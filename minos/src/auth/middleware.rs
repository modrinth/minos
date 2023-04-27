use std::rc::Rc;

use crate::routes::ApiError;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::extractors::{bearer, AuthenticationError};
use futures::future::{ready, LocalBoxFuture, Ready};
use futures::FutureExt;
use http::header::COOKIE;
use ory_client::apis::configuration::Configuration;
use ory_client::apis::frontend_api::to_session;
use ory_client::models::Session;
use reqwest::Client;

use super::AuthError;

pub struct Authenticator;

pub struct AuthenticatorMiddleware<S> {
    service: Rc<S>,
    configuration: Rc<Configuration>,
}

impl<S, B> Transform<S, ServiceRequest> for Authenticator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticatorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // Create new instance of auth middleware
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticatorMiddleware {
            service: Rc::new(service),
            configuration: Rc::new(Configuration {
                api_key: None,
                base_path: dotenvy::var("ORY_URL").unwrap(),
                client: Client::new(),
                basic_auth: None,
                user_agent: Some("Modrinth Minos authenticator".to_string()),
                oauth_access_token: None,
                bearer_access_token: None,
            }),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for AuthenticatorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    // Call middleware
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Clone the Rc pointers so we can move them into the async block.
        let config = self.configuration.clone();
        let srv = self.service.clone();

        async move {
            // Validate session, and if it exists, insert it into the request
            // It can be accessed in-route with "session: Option<web::ReqData<Session>>"
            // let session_result : Result<Session, ApiError> = get_authenticated_session(&config,&req).await;
            let session_result: Result<Session, ApiError> =
                get_authenticated_session(&config, &req)
                    .await
                    .map_err(|e| e.into());
            let session = session_result?;
            req.extensions_mut().insert(session);

            // Continue
            let res = srv.call(req).await?;
            Ok(res)
        }
        .boxed_local()
    }
}

// Authenticate a ServiceRequest by trying to create an Ory session from the Cookie, or the Session token
// We make two separate checks:
// 1. If a cookie is present, we try to create a session from it
// 2. If a session token is passed in the Authorization header (as a Bearer token), we try to create a session from it
// That way Minos endpoints can be called from a browser, or from a API
async fn get_authenticated_session(
    configuration: &Configuration,
    req: &ServiceRequest,
) -> Result<Session, AuthError> {
    // Cookie
    // Do not parse cookies, simply pass them through directly to GET call inside to_session
    let cookies_unparsed = req.headers().get(COOKIE).and_then(|c| c.to_str().ok());
    if cookies_unparsed.is_none() {
        return Err(AuthError::NoMethodFound);
    }
    let session = to_session(configuration, None, cookies_unparsed).await?;
    Ok(session)
}

// Admin API requests need Bearer matching ORY_AUTH_BEARER
pub async fn admin_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req
        .app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default();

    if credentials.token() == dotenvy::var("ORY_AUTH_BEARER").unwrap() {
        Ok(req)
    } else {
        Err((AuthenticationError::from(config).into(), req))
    }
}
