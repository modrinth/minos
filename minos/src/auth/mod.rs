pub mod middleware;

use reqwest::header::ToStrError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Environment Error")]
    Env(#[from] dotenvy::Error),
    #[error("Error while deserializing: {0}")]
    JSON(#[from] serde_json::Error),
    #[error("Could not make a session: {0}")]
    SessionError(#[from] ory_client::apis::Error<ory_client::apis::frontend_api::ToSessionError>),
    #[error("Could not parse cookie: {0}")]
    CookieParseError(#[from] actix_web::cookie::ParseError),
    #[error("Actix error: {0}")]
    ActixError(#[from] actix_web::Error),
    #[error("No cookie found attached to request.")]
    NoCookieError,
    #[error("Could not convert header to string: {0}")]
    HeaderToStrError(#[from] ToStrError),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl actix_web::ResponseError for AuthError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AuthError::Env(..) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::ActixError(..) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::JSON(..) => actix_web::http::StatusCode::BAD_REQUEST,
            AuthError::Reqwest(..) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,

            AuthError::SessionError(..) => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::NoCookieError => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::CookieParseError(..) => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::HeaderToStrError(..) => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(crate::error::ApiError {
            error: match self {
                AuthError::Env(..) => "environment_error",
                AuthError::JSON(..) => "invalid_input",
                AuthError::SessionError(..) => "no_session",
                AuthError::NoCookieError => "invalid_input",
                AuthError::CookieParseError(..) => "invalid_input",
                AuthError::HeaderToStrError(..) => "invalid_input",
                AuthError::Reqwest(..) => "internal_error",
                AuthError::ActixError(..) => "internal_error",
            },
            description: &self.to_string(),
        })
    }
}
