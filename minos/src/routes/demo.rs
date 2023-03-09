use crate::routes::ApiError;
use actix_web::{get, web, HttpResponse};
use http::header::COOKIE;
use ory_client::apis::{configuration::Configuration, frontend_api::to_session};

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
