use std::collections::HashMap;

use actix_web::web;
use ory_client::{apis::configuration::Configuration, models::{JsonPatch, json_patch::OpEnum, Identity, UpdateIdentityBody}};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::pool;

use crate::routes::{ApiError, OryError, user::MinosSessionMetadataPublic};

use super::import::LabrinthUser;

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload{
    id: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OidcDataConfig {
    providers : Vec<OidcDataProvider>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OidcDataProvider {
    subject : String,
    provider : String,
    initial_access_token : String,
    initial_refresh_token : String
}

// POST /admin/oidc-callback
// Updates the OIDC data for an identity with the latest data from the OIDC provider
// Used as a callback after the settings flow
#[actix_web::post("oidc-callback")]
pub async fn oidc_reload(
    payload: web::Json<Payload>,
    pool: web::Data<pool::Pool<sqlx::Postgres>>,
    configuration: web::Data<Configuration>,
) -> Result<actix_web::HttpResponse, ApiError> {
    
    println!("oidc_reload: {:?}", payload);
    // Get the oidc data from database
    let identity_with_credentials = ory_client::apis::identity_api::get_identity(&configuration, &payload.id, Some(vec!["oidc".to_string()])).await.map_err(OryError::from)?;
    let err = || OryError::MissingIdentityData("OIDC".to_string());
    let oidc_data : OidcDataConfig = serde_json::from_value(identity_with_credentials.credentials.clone().ok_or_else(err)?.get("oidc").ok_or_else(err)?.config.clone().ok_or_else(err)?)?;
    let providers = oidc_data.providers;
    let get_new_provider_id = |p: String| {
        providers.iter().find( |x| x.provider == p).map(|x| x.subject.clone())
    };

    dbg!(&identity_with_credentials);

    // Overwite into current metadata_public
    let metadata_public: MinosSessionMetadataPublic = serde_json::from_value(identity_with_credentials.metadata_public.clone().ok_or_else(|| OryError::MissingIdentityData("OIDC".to_string()))?)?;
    
    dbg!(&metadata_public);

    // If we are ADDING a github field and it is not already set, 
    // We need to specifically check if a legacy account exists, as we only allow
    // legacy github account linking through *registration*, not through the settings flow
    if metadata_public.github_id.is_none() && get_new_provider_id("github".to_string()).is_some() {
        dbg!(&metadata_public.github_id, get_new_provider_id("github".to_string()).is_some());

        if check_github_exists(get_new_provider_id("github".to_string()).unwrap_or_default()).await? {
            println!("Github exists in labrinth.");
            // Directly remove the github OIDC just added
            // Must use direct replacement as patching is not supported for credentials
            let identity_with_credentials = remove_github_credentials_from_identity(identity_with_credentials).await;



            dbg!(&identity_with_credentials);
            println!("{}",serde_json::to_string_pretty(&identity_with_credentials)?);
            // Identity with credentials and UpdateIdentityBody have identical json representations, so we can convert between them
            let update_identity_body = serde_json::from_value::<UpdateIdentityBody>(serde_json::to_value(identity_with_credentials)?)?;
            dbg!(&update_identity_body);
            println!("{}",serde_json::to_string_pretty(&update_identity_body)?);
            ory_client::apis::identity_api::update_identity(&configuration, &payload.id, Some(&update_identity_body)).await.map_err(OryError::from)?; 
            return Err(ApiError::Oidc("This Github ID corresponds to an existing legacy account. The OIDC connection ha been retroactively removed. You can link to it only by direct registration.".to_string()));
        }
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
    let json_patch  = JsonPatch {
        from: Some("/metdata_public".to_string()),
        op: OpEnum::Replace,
        path: "/metadata_public".to_string(),
        value: Some(serde_json::to_value(metadata_public)?),
    };
    ory_client::apis::identity_api::patch_identity(&configuration, &payload.id, Some(vec![json_patch])).await.map_err(OryError::from)?;

    // New identity_with_credentials
    let identity_with_credentials = ory_client::apis::identity_api::get_identity(&configuration, &payload.id, Some(vec!["oidc".to_string()])).await.map_err(OryError::from)?;
    Ok(actix_web::HttpResponse::Ok().json(identity_with_credentials))
}

// If the github_id corresponds to an existing account in Labrinth, it should not be addable as a OIDC account in settings
// (This would require account merging, which is avoided)
// To get access to the account, the user should use Github in the registration flow
async fn check_github_exists(id : String) -> Result<bool, ApiError> {
        // Reqwest check to labrinth
        let client = reqwest::Client::new();
        let res : Option<LabrinthUser> = client
            .get(format!("{}/admin/_legacy_account/{id}", dotenvy::var("LABRINTH_API_URL").unwrap()))
            .header("Accept", "application/json")
            .header("Accept-Language", "en_US")
            .header("Modrinth-Admin", dotenvy::var("LABRINTH_ADMIN_KEY").unwrap())
            .send()
            .await?.json().await?;
        dbg!("Github check", &res);
        Ok(res.is_some())
}



#[derive(Deserialize, Serialize)]
struct IdentityCredentials {
    providers: Vec<Provider>
}
#[derive(Deserialize, Serialize)]
struct Provider {
    provider: String,
    subject: String,
    initial_id_token: String,
    initial_access_token: String,
    initial_refresh_token: String,
}
async fn remove_github_credentials(configuration: &Configuration, kratos_id : String, pool : &pool::Pool<sqlx::Postgres>  ) -> Result<(), ApiError> {
    let transaction = pool.begin().await?;

    let record = sqlx::query!(
        "
        SELECT config, ic.id
        FROM identity_credentials ic 
        LEFT JOIN identity_credential_types ict ON ic.identity_credential_type_id = ict.id
        WHERE identity_id = $1 AND ict.name = 'oidc'
        ",
        uuid::Uuid::parse_str(&kratos_id)?,
    )
    .fetch_one(&mut transaction)
    .await?;
    let config = record.config as serde_json::Value;
    let ic_id = record.ic.id as uuid::Uuid;
    let config = serde_json::from_value::<IdentityCredentials>(config)?;

    // Remove the github identifiers ('provider')
    let config = IdentityCredentials {
        providers: config.providers.into_iter().filter(|provider| provider.provider != "github").collect::<Vec<Provider>>()
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
    ).execute(&mut *transaction).await?;


    // delete records from identity_credential_identifiers where the column identity_credential's id kratos id, and identity_credential_type_id links to oidc
    // and lastly where the identifier starts with 'github'
    sqlx::query!(
        "
        ",
        uuid::Uuid::parse_str(&kratos_id)?,
    ).execute(&mut *transaction).await?;


    sqlx::query!(
        "
        DELETE FROM files
        WHERE files.version_id = $1
        ",
        id as VersionId,
    )
    .execute(&mut *transaction)
    .await?;


    Ok(())
}


async fn remove_github_credentials_from_identity(mut i: Identity) -> Identity {
    let providers = i.credentials.as_mut().and_then(|credentials| credentials.get_mut("oidc")).and_then(| oidc | oidc.config.as_mut() ).and_then(|config| config.get_mut("providers")).and_then(|providers| providers.as_array_mut());
    if let Some( providers) = providers {
        providers.retain(|provider| provider.get("provider").and_then(|provider| provider.as_str()).map(|provider| provider != "github").unwrap_or(true));
    }
    i
}
