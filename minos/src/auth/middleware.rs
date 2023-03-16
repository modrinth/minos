use std::rc::Rc;

use actix_web::dev::{Transform, ServiceRequest, Service, ServiceResponse, forward_ready};
use actix_web::Error;
use futures::FutureExt;
use futures::future::{Ready, ready, LocalBoxFuture};
use http::header::COOKIE;
use ory_client::apis::configuration::Configuration;
use ory_client::apis::frontend_api::to_session;
use ory_client::models::Session;
use reqwest::Client;
use super::AuthError;

pub struct Authenticator;

pub struct AuthenticatorMiddleware<S> {
    service: Rc<S>,
    configuration : Rc<Configuration>
}

impl <S,B> AuthenticatorMiddleware<S> 
where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static, 
{
}


impl<S,B> Transform<S, ServiceRequest> for Authenticator
where S: Service<ServiceRequest, Response=ServiceResponse<B>, Error = Error> + 'static,
S::Future: 'static,
B: 'static,{
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
        })}))
    }
}

impl <S,B> Service<ServiceRequest> for AuthenticatorMiddleware<S> 
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
        // let srv = self.service.clone();
        // let auth_data = self.auth_data.clone();
        let config = self.configuration.clone();
        let srv = self.service.clone();
        async move {
            get_authenticated_session(&config,&req).await?;
            let res = srv.call(req).await?;
            Ok(res)    
        }.boxed_local()
    }    
}

// Authenticate a ServiceRequest by trying to create an Ory session from the Cookie
async fn get_authenticated_session(configuration: &Configuration, req: &ServiceRequest) -> Result<Session,AuthError> {
    // Do not parse cookies, simply pass them through directly to GET call inside to_session
    let cookies_unparsed = req.headers().get(COOKIE).ok_or(AuthError::NoCookieError)?;
    let cookies_unparsed = Some(cookies_unparsed.to_str()?);

    // Get session from auth cookie. If this returns a session, there is indeed a session and the user is logged in.
    let session = to_session(configuration, None, cookies_unparsed).await?;
    Ok(session)
}
