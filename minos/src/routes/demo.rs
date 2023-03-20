use crate::routes::ApiError;
use actix_web::{get, web, HttpResponse};
use ory_client::models::Session;
use serde::{Deserialize, Serialize};

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

// TEMPORARY ENDPOINT. TODO: DELETE THIS.
// A temporary endpoint that deletes all users.
// This should *only* be used for testing purpoes.
#[get("delete_all")]
pub async fn delete_all() -> Result<HttpResponse, ApiError> {
    let client = reqwest::Client::new();
    let identities_route = format!("{}/admin/identities", dotenvy::var("ORY_URL").unwrap());
    let ory_admin_bearer = format!("Bearer {}", dotenvy::var("ORY_AUTH_BEARER").unwrap());
    let res = client
        .get(&identities_route)
        .header("Authorization", &ory_admin_bearer)
        .send()
        .await?;
    let identities: Vec<Identity> = res.json().await?;

    for identity in identities.iter() {
        let _res = client
            .delete(format!("{}/{}", identities_route, identity.id))
            .header("Authorization", &ory_admin_bearer)
            .send()
            .await?;
    }

    Ok(HttpResponse::Ok().json(identities))
}
