use crate::routes::{ApiError, OryError};
use actix_web::{post, web, HttpResponse};
use ory_client::{
    apis::{configuration::Configuration, frontend_api},
    models::UpdateLoginFlowBody,
};
use serde::{Deserialize, Serialize};

// POST /login
// Makes a new session, and returns it.
// This should not be used except as an API endpoint, such as via Labrinth (Pasiphae should use the Ory Kratos API return links directly)
// This should NOT be called from the browser, as it uses the Ory Kratos native endpoint, and does not have CSRF protection.
#[derive(Deserialize)]
pub enum LoginRequest {
    Password {
        email: String,
        password: String,
        // If we want to refresh as session, we pass this
        refreshable_x_session: Option<String>,
    },
    Oauth2 {
        provider: String,
        email: String,
        // If we want to refresh as session, we pass this
        refreshable_x_session: Option<String>,
    },
}

#[post("login")]
pub async fn api_login(
    body: web::Json<LoginRequest>,
    configuration: web::Data<Configuration>,
) -> Result<HttpResponse, ApiError> {
    // If we choose to refresh a session/cookies, we need to get the cookies from the request
    let refreshable_x_session = match &*body {
        LoginRequest::Password {
            refreshable_x_session,
            ..
        } => refreshable_x_session,
        LoginRequest::Oauth2 {
            refreshable_x_session,
            ..
        } => refreshable_x_session,
    };

    // Create login flow
    let flow = frontend_api::create_native_login_flow(
        &configuration,
        Some(refreshable_x_session.is_some()),
        None,
        refreshable_x_session.as_deref(),
    )
    .await
    .map_err(OryError::from)?;

    let update_login_flow_body = match &*body {
        LoginRequest::Password {
            email, password, ..
        } => {
            UpdateLoginFlowBody::UpdateLoginFlowWithPasswordMethod {
                csrf_token: None,
                identifier: email.clone(),
                password: password.clone(),
                password_identifier: None, // deprecated
            }
        }
        LoginRequest::Oauth2 {
            provider, email, ..
        } => UpdateLoginFlowBody::UpdateLoginFlowWithOidcMethod {
            csrf_token: None,
            provider: provider.to_string(),
            traits: Some(
                serde_json::to_value(ImportedUserTraits {
                    email: email.clone(),
                })
                .map_err(OryError::from)?,
            ),
            upstream_parameters: None,
        },
    };

    let flow = frontend_api::update_login_flow(
        &configuration,
        &flow.id,
        &update_login_flow_body,
        refreshable_x_session.as_deref(),
        None,
    )
    .await
    .map_err(OryError::from)?;
    Ok(HttpResponse::Ok().json(&flow.session))
}

#[derive(Serialize, Debug)]
pub struct ImportedUserTraits {
    pub email: String,
}
