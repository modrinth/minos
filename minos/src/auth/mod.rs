pub mod middleware;

use reqwest::header::ToStrError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Environment Error")]
    Env(#[from] dotenvy::Error),
    #[error("Error while deserializing: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Could not make a session: {0}")]
    SessionError(#[from] ory_client::apis::Error<ory_client::apis::frontend_api::ToSessionError>),
    #[error("Could not parse cookie: {0}")]
    CookieParseError(#[from] actix_web::cookie::ParseError),
    #[error("Actix error: {0}")]
    ActixError(#[from] actix_web::Error),
    #[error("Could not convert header to string: {0}")]
    HeaderToStrError(#[from] ToStrError),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("No cookie or Bearer header found attached to request.")]
    NoMethodFound,
}
