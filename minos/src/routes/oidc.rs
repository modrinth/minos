use actix_web::web;
use ory_client::{apis::configuration::Configuration, models::{JsonPatch, json_patch::OpEnum}};
use serde::{Deserialize, Serialize};

use crate::routes::{ApiError, OryError, user::MinosSessionMetadataPublic};

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
    configuration: web::Data<Configuration>,
) -> Result<actix_web::HttpResponse, ApiError> {
    
    // Get the oidc data from database
    let identity_with_credentials = ory_client::apis::identity_api::get_identity(&configuration, &payload.id, Some(vec!["oidc".to_string()])).await.map_err(OryError::from)?;
    let err = || OryError::MissingIdentityData("OIDC".to_string());
    let oidc_data : OidcDataConfig = serde_json::from_value(identity_with_credentials.credentials.ok_or_else(err)?.get("oidc").ok_or_else(err)?.config.clone().ok_or_else(err)?)?;
    let providers = oidc_data.providers;

    // Overwite into current metadata_public
    let metadata_public: MinosSessionMetadataPublic = serde_json::from_value(identity_with_credentials.metadata_public.ok_or_else(|| OryError::MissingIdentityData("OIDC".to_string()))?)?;

    let get_new_provider_id = |p: String| {
        providers.iter().find( |x| x.provider == p).map(|x| x.subject.clone())
    };

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

