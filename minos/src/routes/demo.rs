use crate::routes::ApiError;
use actix_web::{get, web, HttpResponse};
use ory_client::models::Session;
use serde::{Deserialize, Serialize};

// GET /user/demo
// Demo endpoint for Minos
// Returns 200, and returns the authenticated session.
// This is encapsuled by the authentication middleware (returning 401 in middleware if it fails to authenticate)
#[get("demo")]
pub async fn demo_get(session: Option<web::ReqData<Session>>) -> Result<HttpResponse, ApiError> {
    let session = session.ok_or(ApiError::SessionError)?;
    Ok(HttpResponse::Ok().json(&*session))
}

#[derive(Deserialize, Serialize)]
pub struct Identity {
    id: String,
}
