use crate::auth::AuthError;
use thiserror::Error;

pub mod demo;
pub mod not_found;

pub use not_found::not_found;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Environment Error")]
    Env(#[from] dotenvy::Error),
    #[error("Authentication error: {0}")]
    Unauthorized(#[from] AuthError),
    #[error("Error while deserializing: {0}")]
    JSON(#[from] serde_json::Error),
    #[error("Failed to insert authentication session into request")]
    SessionError,
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::Env(..) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::JSON(..) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::SessionError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Reqwest(..) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,

            ApiError::Unauthorized(..) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(crate::error::ApiError {
            error: match self {
                ApiError::Env(..) => "environment_error",
                ApiError::JSON(..) => "invalid_input",
                ApiError::SessionError => "internal_error",
                ApiError::Reqwest(..) => "internal_error",

                ApiError::Unauthorized(..) => "unauthorized",
            },
            description: &self.to_string(),
        })
    }
}
