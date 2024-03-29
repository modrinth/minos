use crate::{
    routes::{ApiError, OryError},
    util::ory::AdminConfiguration,
};
use actix_web::{get, web, HttpResponse};
use ory_client::{
    apis::identity_api::{self, get_identity},
    models::{Identity, Session},
};
use serde::{Deserialize, Serialize};

// GET /user/session
// Returns 200, and returns the authenticated session.
// This is encapsuled by the authentication middleware (returning 401 in middleware if it fails to authenticate)
#[get("session")]
pub async fn user_session_get(
    session: Option<web::ReqData<Session>>,
) -> Result<HttpResponse, ApiError> {
    let session = session.ok_or(ApiError::SessionError)?;
    Ok(HttpResponse::Ok().json(&*session))
}

/*
   Relevant Minos Session JSON example:
   {
       "id": "4ab8afc4-25e0-46c8-8fdb-1d3d86bd41bd",
       "identity": {
           "id": "8e1b56c4-80fe-4d6e-8251-61f6657a554a",
           "traits": {
           "email": "hellotest@gmail.com",
           "username": "hellotest"
           },
       },
   }
*/

// GET /user
// Returns 200, and returns the authenticated user as a MinosUser object (a simplified version of the Ory Kratos User object)
#[derive(Serialize, Deserialize, Debug)]
pub struct MinosSessionTraits {
    pub email: String,
    pub username: String,
    pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MinosSessionMetadataPublic {
    pub github_id: Option<String>,
    pub discord_id: Option<String>,
    pub google_id: Option<String>,
    pub gitlab_id: Option<String>,
    pub microsoft_id: Option<String>,
    pub apple_id: Option<String>,
    pub default_picture: Option<String>,
}

// MinosUser is a simplified version of the Ory Kratos User object
// This is used as a communication struct to Labrinth
// It must be identical to the MinosUser struct in Labrinth, as well
// as the one defined in minos_user.jsonnet
#[derive(Serialize, Deserialize, Debug)]
pub struct MinosUser {
    pub id: String,       // This is the unique generated Ory name
    pub username: String, // unique username
    pub email: String,
    pub name: Option<String>, // real name
    pub default_picture: Option<String>,
    pub github_id: Option<u64>,
    pub discord_id: Option<u64>,
    pub google_id: Option<u128>,
    pub gitlab_id: Option<u64>,
    pub microsoft_id: Option<u64>,
    pub apple_id: Option<u64>,
}

// /admin/user/{minos_id}
// Protected by admin bearer token, should be accessed by labrinth only (or by admins)
// Get a user as a MinosUser struct (a simplified identity as relevant to labrinth)
#[get("/user/{minos_id}")]
pub async fn user_get_id(
    path: web::Path<String>,
    configuration: web::Data<AdminConfiguration>,
) -> Result<HttpResponse, ApiError> {
    let minos_id = &path.into_inner();
    let identity = get_identity(&configuration.0, minos_id, None)
        .await
        .map_err(OryError::from)?;
    let minos_user = extract_minos_user(&identity)?;
    Ok(HttpResponse::Ok().json(minos_user))
}

// /admin/user/token
// Protected by admin bearer token, should be accessed by labrinth only (or by admins)
// Get a user as a MinosUser struct (a simplified identity as relevant to labrinth)
// User identified is the *session token passed*. If Ory session token is "12345-12345",
// you can use "ory_12345-12345" as a Labrinth token to get the user here.
// (User should be able to pass the cookie info as a bearer token, and this facilitates that)
#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}
#[get("/user/token")]
pub async fn user_get_id_by_token(
    web::Query(token): web::Query<Token>,
    configuration: web::Data<AdminConfiguration>,
) -> Result<HttpResponse, ApiError> {
    // If it starts with "ory_", remove it to extract raw session ID
    let token = Token {
        token: token.token.trim_start_matches("ory_").to_string(),
    };
    let session = identity_api::get_session(
        &configuration.0,
        &token.token,
        Some(vec!["Identity".to_string()]),
    )
    .await
    .map_err(OryError::from)?;
    let minos_user = extract_minos_user(&session.identity)?;
    Ok(HttpResponse::Ok().json(minos_user))
}

// /user
// Get a user as a MinosUser struct (a simplified identity as relevant to labrinth)
#[get("")]
pub async fn user_get(session: Option<web::ReqData<Session>>) -> Result<HttpResponse, ApiError> {
    let session = session.ok_or(ApiError::SessionError)?;
    let identity = &session.identity;
    let minos_user = extract_minos_user(identity)?;
    Ok(HttpResponse::Ok().json(minos_user))
}

fn extract_minos_user(identity: &Identity) -> Result<MinosUser, ApiError> {
    let metadata_public: MinosSessionMetadataPublic =
        if let Some(m) = identity.metadata_public.clone() {
            serde_json::from_value(m)?
        } else {
            Default::default()
        };
    let traits: MinosSessionTraits = serde_json::from_value(
        identity
            .traits
            .as_ref()
            .ok_or_else(|| ApiError::SessionError)?
            .clone(),
    )?;

    Ok(MinosUser {
        id: identity.id.clone(),
        username: traits.username,
        email: traits.email,
        name: traits.name,
        default_picture: metadata_public.default_picture,
        // Parse as i64 or propogate parsing error outwards
        github_id: metadata_public.github_id.map(|s| s.parse()).transpose()?,
        discord_id: metadata_public.discord_id.map(|s| s.parse()).transpose()?,
        google_id: metadata_public.google_id.map(|s| s.parse()).transpose()?,
        gitlab_id: metadata_public.gitlab_id.map(|s| s.parse()).transpose()?,
        microsoft_id: metadata_public
            .microsoft_id
            .map(|s| s.parse())
            .transpose()?,
        apple_id: metadata_public.apple_id.map(|s| s.parse()).transpose()?,
    })
}
