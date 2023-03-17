use crate::routes::ApiError;
use actix_web::{get, web, HttpResponse};
use http::header::COOKIE;
use ory_client::apis::{configuration::Configuration, frontend_api::to_session};
use serde::{Deserialize, Serialize};

// Demo endpoint for Minos
// Returns 200 if and only if:
// -> access is not blocked by CORs
// -> a Cookie is attached with a proper Ory session
// In the future, this functionality will actually be abstracted by a middleware.
#[get("demo")]
pub async fn demo_get(
    req: actix_web::HttpRequest,
    configuration: web::Data<Configuration>,
) -> Result<HttpResponse, ApiError> {
    // Do not parse cookies, simply pass them through directly to GET call inside to_session
    let cookies_unparsed = req.headers().get(COOKIE).ok_or(ApiError::NoCookieError)?;
    let cookies_unparsed = Some(cookies_unparsed.to_str()?);

    // Get session from auth cookie. If this returns a session, there is indeed a session and the user is logged in.
    let session = to_session(&configuration, None, cookies_unparsed).await?;
    Ok(HttpResponse::Ok().json(session))
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
