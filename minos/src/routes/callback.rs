use actix_web::web;
use serde::{Deserialize, Serialize};

use sqlx::pool;

use crate::{
    routes::{ApiError, OryError},
    util::{callback::CallbackError, email, oidc, ory::AdminConfiguration},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    identity_id: String,
    flow_id: String,
}

// POST /admin/settings-callback
// Performs manual adjustments to acccount after settings flow update
// -> Updates the OIDC data for an identity with the latest data from the OIDC provider
// Used as a callback after the settings flow- designed to directly interface with Ory Kratos (as Ory doesnt have support for our unique usecase here)
#[actix_web::post("settings-callback")]
pub async fn settings_callback(
    payload: web::Json<Payload>,
    pool: web::Data<pool::Pool<sqlx::Postgres>>,
    configuration: web::Data<AdminConfiguration>,
) -> Result<actix_web::HttpResponse, CallbackError> {
    println!("Settings callback: {:?}", payload);
    let identity_with_credentials = ory_client::apis::identity_api::get_identity(
        &configuration.0,
        &payload.identity_id,
        Some(vec!["oidc".to_string()]),
    )
    .await
    .map_err(OryError::from)
    .map_err(ApiError::from);
    
    dbg!(&identity_with_credentials);
    let identity_with_credentials = identity_with_credentials?;


    // Handle OIDC:
    let a = oidc::oidc_reload(&identity_with_credentials, &pool, &configuration).await.map_err(CallbackError::from);
    dbg!(&a);
    let a = a?;

    // Update email:
    let a = email::email_update(&identity_with_credentials).await;
    dbg!(&a);
    let a = a?;

    Ok(actix_web::HttpResponse::Ok().json(identity_with_credentials))
}
