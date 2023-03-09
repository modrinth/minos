use reqwest::header::ToStrError;
use thiserror::Error;

pub mod demo;
pub mod not_found;

pub use not_found::not_found;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Environment Error")]
    Env(#[from] dotenvy::Error),
    #[error("Error while deserializing: {0}")]
    JSON(#[from] serde_json::Error),
    #[error("Could not make a session: {0}")]
    SessionError(#[from] ory_client::apis::Error<ory_client::apis::frontend_api::ToSessionError>),
    #[error("Could not parse cookie: {0}")]
    CookieParseError(#[from] actix_web::cookie::ParseError),
    #[error("No cookie found attached to request.")]
    NoCookieError,
    #[error("Could not convert header to string: {0}")]
    HeaderToStrError(#[from] ToStrError),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::Env(..) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::JSON(..) => actix_web::http::StatusCode::BAD_REQUEST,

            ApiError::SessionError(..) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::NoCookieError => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::CookieParseError(..) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::HeaderToStrError(..) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::Reqwest(..) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(crate::error::ApiError {
            error: match self {
                ApiError::Env(..) => "environment_error",
                ApiError::JSON(..) => "invalid_input",
                ApiError::SessionError(..) => "no_session",
                ApiError::NoCookieError => "invalid_input",
                ApiError::CookieParseError(..) => "invalid_input",
                ApiError::HeaderToStrError(..) => "invalid_input",
                ApiError::Reqwest(..) => "internal_error",
            },
            description: &self.to_string(),
        })
    }
}
