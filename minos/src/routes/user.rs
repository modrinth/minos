use crate::routes::{ApiError, OryError};
use actix_web::{get, web, HttpResponse};
use ory_client::{
    apis::{configuration::Configuration, identity_api},
    models::{Identity, Session},
};
use serde::{Deserialize, Serialize};

// GET /user/session
// Demo endpoint for Minos
// Returns 200, and returns the authenticated session.
// This is encapsuled by the authentication middleware (returning 401 in middleware if it fails to authenticate)
#[get("session")]
pub async fn user_get(session: Option<web::ReqData<Session>>) -> Result<HttpResponse, ApiError> {
    let session = session.ok_or(ApiError::SessionError)?;
    Ok(HttpResponse::Ok().json(&*session))
}

// GET /admin/identities
// Test endpoint for Minos, returns all identities
#[derive(Deserialize, Serialize, Debug)]
pub struct EmailPayload {
    pub email: String,
}
#[get("identities")]
pub async fn list_identities(
    configuration: web::Data<Configuration>,
) -> Result<HttpResponse, ApiError> {
    let identities: Vec<Identity> = identity_api::list_identities(&configuration, None, None, None)
        .await
        .map_err(OryError::from)?;

    Ok(HttpResponse::Ok().json(identities))
}
