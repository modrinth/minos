use crate::{
    auth::{middleware::Authenticator, AuthError},
    error,
};
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use thiserror::Error;

pub mod delete;
pub mod import;
pub mod not_found;
pub mod user;

pub use not_found::not_found;

// User routes
// Protected by auth middleware- requires a valid session cookie or Bearer token (representing a session)
pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("user")
            .service(user::user_get)
            .service(user::user_session_get)
            .wrap(Authenticator), // Auth middleware
    );
}

// Admin routes
// Interface directly to Ory API
// Requires admin Bearer token to be passed to access these
pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("admin")
            .service(import::import_account)
            .service(import::pull_labrinth_github_accounts)
            .service(delete::delete_all)
            .wrap(HttpAuthentication::bearer(
                crate::auth::middleware::admin_validator,
            )),
    );
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Environment Error")]
    Env(#[from] dotenvy::Error),
    #[error("Authentication error: {0}")]
    Unauthorized(#[from] AuthError),
    #[error("Ory error: {0}")]
    Ory(#[from] OryError),
    #[error("Error while deserializing: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Failed to insert authentication session into request")]
    SessionError,
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::Env(..) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Json(..) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::Ory(..) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::SessionError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Reqwest(..) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,

            ApiError::Unauthorized(..) => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(crate::error::ApiError {
            error: match self {
                ApiError::Env(..) => "environment_error",
                ApiError::Json(..) => "invalid_input",
                ApiError::Ory(..) => "invalid_input",
                ApiError::SessionError => "internal_error",
                ApiError::Reqwest(..) => "internal_error",
                ApiError::Unauthorized(..) => "unauthorized",
            },
            description: &self.to_string(),
        })
    }
}

#[derive(Error, Debug)]
pub enum OryError {
    #[error("Create Identity error: {0}")]
    CreateIdentityError(
        #[from] ory_client::apis::Error<ory_client::apis::identity_api::CreateIdentityError>,
    ),
    #[error("List Identity error: {0}")]
    ListIdentitiesError(
        #[from] ory_client::apis::Error<ory_client::apis::identity_api::ListIdentitiesError>,
    ),
    #[error("Delete Identity error: {0}")]
    DeleteIdentityError(
        #[from] ory_client::apis::Error<ory_client::apis::identity_api::DeleteIdentityError>,
    ),
    #[error("Create login flow error: {0}")]
    CreateLoginFlowError(
        #[from] ory_client::apis::Error<ory_client::apis::frontend_api::CreateNativeLoginFlowError>,
    ),
    #[error("Update login flow error: {0}")]
    UpdateLoginFlowError(
        #[from] ory_client::apis::Error<ory_client::apis::frontend_api::UpdateLoginFlowError>,
    ),

    #[error("Error while deserializing: {0}")]
    Json(#[from] serde_json::Error),
}
