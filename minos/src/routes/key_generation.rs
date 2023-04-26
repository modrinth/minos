use crate::routes::{ApiError, OryError};
use actix_web::{post, web, HttpResponse};
use http::StatusCode;
use ory_client::{
    apis::{
        configuration::{self, Configuration},
        frontend_api::{self, CreateNativeLoginFlowError},
        ResponseContent,
    },
    models::UpdateLoginFlowBody,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ImportedUserTraits {
    pub email: String,
}

// POST /generate_key
// Generates a new API key (by going through the login flow) for use in API connecting
// This should not be used except as an API endpoint, such as via Labrinth (Pasiphae should use the Ory Kratos API return links directly)
// This should NOT be called from a browser, as it uses the Ory Kratos native endpoint, and does not have CSRF protection.
#[derive(Deserialize)]
#[serde(untagged)]
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
#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub session_id: String,
    // These are Option<> for consistency with Ory returned data
    pub active: Option<bool>,
    pub api_key: Option<String>,
    pub authenticated_at: Option<String>,
    pub expires_at: Option<String>,
}

#[post("generate_key")]
pub async fn api_keygen(
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
    let flow = create_native_login_flow(
        &configuration,
        Some(refreshable_x_session.is_some()),
        None,
        refreshable_x_session.as_deref(),
    )
    .await
    .map_err(OryError::from)?;

    // Create UpdateLoginFlowBody and update login flow, logging in to Ory kratos
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
    .map_err(|e| {
        if let ory_client::apis::Error::ResponseError(e) = &e {
            if e.status == StatusCode::BAD_REQUEST {
                // Ory sends 400 if the credentials are invalid
                return ApiError::InvalidCredentials;
            }
        }
        ApiError::from(OryError::from(e))
    })?;
    Ok(HttpResponse::Ok().json(LoginResponse {
        active: flow.session.active,
        api_key: flow.session_token,
        session_id: flow.session.id,
        authenticated_at: flow.session.authenticated_at,
        expires_at: flow.session.expires_at,
    }))
}

// This endpoint REPLACES the currently bugged Ory Kratos API endpoint of the same name.
// It no longer gives an error deserializing to an invalid unintended struct.
// The only change made is the return type to a valid deserializable struct that matches what Ory Kratos returns (and is relevant to us).
pub async fn create_native_login_flow(
    configuration: &configuration::Configuration,
    refresh: Option<bool>,
    aal: Option<&str>,
    x_session_token: Option<&str>,
) -> Result<LoginFlow, ory_client::apis::Error<CreateNativeLoginFlowError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/self-service/login/api", configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = refresh {
        local_var_req_builder =
            local_var_req_builder.query(&[("refresh", local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = aal {
        local_var_req_builder = local_var_req_builder.query(&[("aal", local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_session_token {
        local_var_req_builder =
            local_var_req_builder.header("X-Session-Token", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(ory_client::apis::Error::from)
    } else {
        let local_var_entity: Option<CreateNativeLoginFlowError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(ory_client::apis::Error::ResponseError(local_var_error))
    }
}

// Replacement struct matching Ory's API response
// Some especially irrelevant fields are removed for brevity, and improper fields are fixed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginFlow {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<ory_client::models::IdentityCredentialsType>,
    /// ExpiresAt is the time (UTC) when the flow expires. If the user still wishes to log in, a new flow has to be initiated.
    #[serde(rename = "expires_at")]
    pub expires_at: String,
    /// ID represents the flow's unique ID. When performing the login flow, this represents the id in the login UI's query parameter: http://<selfservice.flows.login.ui_url>/?flow=<flow_id>
    #[serde(rename = "id")]
    pub id: String,
    /// RequestURL is the initial URL that was requested from Ory Kratos. It can be used to forward information contained in the URL's path or query for example.
    #[serde(rename = "request_url")]
    pub request_url: String,
    /// ReturnTo contains the requested return_to URL.
    #[serde(rename = "return_to", skip_serializing_if = "Option::is_none")]
    pub return_to: Option<String>,
}
