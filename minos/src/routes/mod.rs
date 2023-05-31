use crate::{
    auth::{middleware::Authenticator, AuthError},
    error,
};
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use thiserror::Error;

pub mod delete;
pub mod not_found;
pub mod oidc;
pub mod user;

pub use not_found::not_found;

// User routes
// Protected by auth middleware- requires a valid session cookie or Bearer token (representing a session)
pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("user")
            .service(user::user_get) // Get user of current session accessing this endpoint
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
            .service(user::user_get_id_by_token)
            .service(user::user_get_id)
            .service(oidc::oidc_reload)
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
    #[error("Failed to parse metadata: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Failed to parse uuid: {0}")]
    ParseUuid(#[from] uuid::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::Env(..) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Json(..) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::Ory(..) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::ParseInt(..) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::SessionError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Reqwest(..) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ParseUuid(..) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(..) => actix_web::http::StatusCode::UNAUTHORIZED,
            ApiError::Database(..) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(crate::error::ApiError {
            error: match self {
                ApiError::Env(..) => "environment_error",
                ApiError::Json(..) => "invalid_input",
                ApiError::ParseInt(..) => "invalid_input",
                ApiError::Ory(..) => "invalid_input",
                ApiError::SessionError => "internal_error",
                ApiError::Reqwest(..) => "internal_error",
                ApiError::Unauthorized(..) => "unauthorized",
                ApiError::ParseUuid(..) => "invalid_input",
                ApiError::Database(..) => "internal_error",
            },
            description: &self.to_string(),
        })
    }
}

#[derive(Error, Debug)]
pub enum OryError {
    #[error("Missing expected Identity data: {0}")]
    MissingIdentityData(String),

    #[error("Create Identity error: {0}")]
    CreateIdentityError(
        #[from] ory_client::apis::Error<ory_client::apis::identity_api::CreateIdentityError>,
    ),
    #[error("Get Identity error: {0}")]
    GetIdentityError(
        #[from] ory_client::apis::Error<ory_client::apis::identity_api::GetIdentityError>,
    ),
    #[error("Patch Identity error: {0}")]
    PatchIdentityError(
        #[from] ory_client::apis::Error<ory_client::apis::identity_api::PatchIdentityError>,
    ),
    #[error("Update Identity error: {0}")]
    UpdateIdentityError(
        #[from] ory_client::apis::Error<ory_client::apis::identity_api::UpdateIdentityError>,
    ),
    #[error("List Identity error: {0}")]
    ListIdentitiesError(
        #[from] ory_client::apis::Error<ory_client::apis::identity_api::ListIdentitiesError>,
    ),
    #[error("Delete Identity error: {0}")]
    DeleteIdentityError(
        #[from] ory_client::apis::Error<ory_client::apis::identity_api::DeleteIdentityError>,
    ),
    #[error("Get Session error: {0}")]
    GetSessionError(
        #[from] ory_client::apis::Error<ory_client::apis::identity_api::GetSessionError>,
    ),
    #[error("Create login flow error: {0}")]
    CreateLoginFlowError(
        #[from] ory_client::apis::Error<ory_client::apis::frontend_api::CreateNativeLoginFlowError>,
    ),
    #[error("Update login flow error: {0}")]
    UpdateLoginFlowError(
        #[from] ory_client::apis::Error<ory_client::apis::frontend_api::UpdateLoginFlowError>,
    ),
    #[error("Get settings flow error: {0}")]
    GetSettingsFlowError(
        #[from] ory_client::apis::Error<ory_client::apis::frontend_api::GetSettingsFlowError>,
    ),
    #[error("Create settings flow error: {0}")]
    CreateSettingsFlowError(
        #[from]
        ory_client::apis::Error<ory_client::apis::frontend_api::CreateNativeSettingsFlowError>,
    ),
    #[error("Update settings flow error: {0}")]
    UpdateSettingsFlowError(
        #[from] ory_client::apis::Error<ory_client::apis::frontend_api::UpdateSettingsFlowError>,
    ),

    #[error("Error while deserializing: {0}")]
    Json(#[from] serde_json::Error),
}
