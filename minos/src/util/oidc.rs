use actix_web::HttpResponse;
use ory_client::{
    apis::configuration::Configuration,
    models::{json_patch::OpEnum, Identity, JsonPatch},
};
use serde::{Deserialize, Serialize};

use sqlx::pool;

use crate::{
    database::models::{
        labrinth::LabrinthUser,
        webhook::{OryMessage, OryWebhookMessagePacket, OryWebhookPayload},
    },
    routes::{user::MinosSessionMetadataPublic, ApiError, OryError},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    identity_id: String,
    // flow_id: String, if needed for future handling of settings flow
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OidcDataConfig {
    providers: Vec<OidcDataProvider>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OidcDataProvider {
    subject: String,
    provider: String,
    initial_access_token: String,
    initial_refresh_token: String,
}

// Updates the OIDC data for an identity with the latest data from the OIDC provider
// Used as a callback after the settings flow- designed to directly interface with Ory Kratos
pub async fn oidc_reload(
    identity_with_credentials: &Identity,
    pool: &pool::Pool<sqlx::Postgres>,
    configuration: &Configuration,
) -> Result<actix_web::HttpResponse, ApiError> {
    // Get the oidc data from database
    let err = || OryError::MissingIdentityData("OIDC".to_string());
    let credentials = identity_with_credentials
        .credentials
        .clone()
        .ok_or_else(err)?;

    // If there is no OIDC data, that's fine. Just return
    let oidc_data = match credentials.get("oidc") {
        Some(o) => o,
        None => return Ok(HttpResponse::Ok().json(identity_with_credentials)),
    };

    let oidc_data: OidcDataConfig =
        serde_json::from_value(oidc_data.config.clone().ok_or_else(err)?)?;
    let providers = oidc_data.providers;
    let get_new_provider_id = |p: String| {
        providers
            .iter()
            .find(|x| x.provider == p)
            .map(|x| x.subject.clone())
    };
    // Overwite into current metadata_public
    let metadata_public: MinosSessionMetadataPublic = serde_json::from_value(
        identity_with_credentials
            .metadata_public
            .clone()
            .ok_or_else(|| OryError::MissingIdentityData("OIDC".to_string()))?,
    )?;

    // If we are ADDING a github field and it is not already set,
    // We need to specifically check if a legacy account exists, as we only allow
    // legacy github account linking through *registration*, not through the settings flow
    if metadata_public.github_id.is_none()
        && get_new_provider_id("github".to_string()).is_some()
        && check_github_exists(get_new_provider_id("github".to_string()).unwrap_or_default())
            .await?
    {
        // Directly remove the github OIDC just added from the db directly
        // Must use direct replacement as patching is not supported for credentials
        remove_github_credentials(&identity_with_credentials.id, pool).await?;
        return Ok(HttpResponse::BadRequest().json(OryWebhookPayload {
            messages: vec![
                OryWebhookMessagePacket {
                    instance_ptr: "github".to_string(),
                    messages: vec![
                        OryMessage {
                            id: 0,
                            text: "This Github account is already linked to an existing legacy Modrinth account. Make a new account with that Github OIDC to gain access to it, rather than merging this account.".to_string(),
                            r#type: "collision".to_string(),
                            context: None
                        }
                    ]
                }
            ]
        }));
    }

    // Update metadata_public to match current provider records
    // explicit so no fields are missed if updated
    let metadata_public = MinosSessionMetadataPublic {
        github_id: get_new_provider_id("github".to_string()),
        discord_id: get_new_provider_id("discord".to_string()),
        google_id: get_new_provider_id("google".to_string()),
        gitlab_id: get_new_provider_id("gitlab".to_string()),
        microsoft_id: get_new_provider_id("microsoft".to_string()),
        apple_id: get_new_provider_id("apple".to_string()),

        default_picture: metadata_public.default_picture,
    };

    // Save OIDC data back as metadata in the identity
    let json_patch = JsonPatch {
        from: Some("/metdata_public".to_string()),
        op: OpEnum::Replace,
        path: "/metadata_public".to_string(),
        value: Some(serde_json::to_value(metadata_public)?),
    };
    ory_client::apis::identity_api::patch_identity(
        configuration,
        &identity_with_credentials.id,
        Some(vec![json_patch]),
    )
    .await
    .map_err(OryError::from)?;

    // New identity_with_credentials
    let identity_with_credentials = ory_client::apis::identity_api::get_identity(
        configuration,
        &identity_with_credentials.id,
        Some(vec!["oidc".to_string()]),
    )
    .await
    .map_err(OryError::from)?;
    Ok(actix_web::HttpResponse::Ok().json(identity_with_credentials))
}

// If the github_id corresponds to an existing account in Labrinth, it should not be addable as a OIDC account in settings
// (This would require account merging, which is avoided)
// To get access to the account, the user should use Github in the registration flow
async fn check_github_exists(id: String) -> Result<bool, ApiError> {
    // Reqwest check to labrinth
    let client = reqwest::Client::new();
    let res: Option<LabrinthUser> = client
        .get(format!(
            "{}/admin/_legacy_account/{id}",
            dotenvy::var("LABRINTH_API_URL").unwrap()
        ))
        .header("Accept", "application/json")
        .header("Accept-Language", "en_US")
        .header(
            "Modrinth-Admin",
            dotenvy::var("LABRINTH_ADMIN_KEY").unwrap(),
        )
        .send()
        .await?
        .json()
        .await?;
    Ok(res.is_some())
}

#[derive(Deserialize, Serialize)]
struct IdentityCredentials {
    providers: Vec<Provider>,
}
#[derive(Deserialize, Serialize)]
struct Provider {
    provider: String,
    subject: String,
    initial_id_token: String,
    initial_access_token: String,
    initial_refresh_token: String,
}

// Ory currently does not support administratively removing credentials, so we must do it manually
// by directly modifying the database
// We update the identity_credentials table to remove the github entry in the JSON config field
// We also remove all github entries in the identity_credentials_identitifiers table
async fn remove_github_credentials(
    kratos_id: &str,
    pool: &pool::Pool<sqlx::Postgres>,
) -> Result<(), ApiError> {
    let mut transaction = pool.begin().await?;

    // Get the identit_credentials_type id for oidc
    let record = sqlx::query!(
        "
        SELECT id
        FROM identity_credential_types
        WHERE name = 'oidc'
        ",
    )
    .fetch_one(&mut transaction)
    .await?;
    let oidc_type_id = record.id as uuid::Uuid;

    let record = sqlx::query!(
        "
        SELECT config, ic.id
        FROM identity_credentials ic 
        WHERE identity_id = $1 AND ic.identity_credential_type_id = $2
        ",
        uuid::Uuid::parse_str(&kratos_id)?,
        oidc_type_id,
    )
    .fetch_one(&mut transaction)
    .await?;
    let config = record.config as serde_json::Value;
    let ic_id = record.id as uuid::Uuid;
    let config = serde_json::from_value::<IdentityCredentials>(config)?;

    // Remove the github identifiers ('provider')
    let config = IdentityCredentials {
        providers: config
            .providers
            .into_iter()
            .filter(|provider| provider.provider != "github")
            .collect::<Vec<Provider>>(),
    };

    // Update it back into the table
    sqlx::query!(
        "
        UPDATE identity_credentials
        SET config = $1
        FROM identity_credential_types ict
        WHERE identity_id = $2 AND ict.name = 'oidc' AND identity_credential_type_id = ict.id
        ",
        serde_json::to_value(config)?,
        uuid::Uuid::parse_str(&kratos_id)?,
    )
    .execute(&mut *transaction)
    .await?;

    // Delete from identity_credential_identifiers all identifiers
    // of type oidc_type_id and starting with 'github'
    sqlx::query!(
        "
        DELETE FROM identity_credential_identifiers
        WHERE identity_credential_id = $1 AND identity_credential_type_id = $2 AND identifier LIKE 'github%'
        ",
        ic_id,
        oidc_type_id,
    ).execute(&mut *transaction).await?;

    transaction.commit().await?;
    Ok(())
}
