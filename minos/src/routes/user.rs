use crate::routes::ApiError;
use actix_web::{get, web, HttpResponse};
use ory_client::models::Session;
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
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MinosSessionMetadataPublic {
    pub github_id: Option<String>,
    pub discord_id: Option<String>,
    pub google_id: Option<String>,
    pub gitlab_id: Option<String>,
    pub microsoft_id: Option<String>,
    pub apple_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinosUser {
    pub id: String,       // This is the unique generated Ory name
    pub username: String, // unique username
    pub email: String,
    pub name: Option<String>, // real name
    pub github_id: Option<i64>,
    pub discord_id: Option<i64>,
    pub google_id: Option<i128>,
    pub gitlab_id: Option<i64>,
    pub microsoft_id: Option<i64>,
    pub apple_id: Option<i64>,
}

#[get("")]
pub async fn user_get(session: Option<web::ReqData<Session>>) -> Result<HttpResponse, ApiError> {
    let session = session.ok_or(ApiError::SessionError)?;
    let identity = &session.identity;
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

    let minos_user = MinosUser {
        id: identity.id.clone(),
        username: traits.username,
        email: traits.email,
        name: None,
        // Parse as i64 or propogate parsing error outwards
        github_id:  metadata_public.github_id.map(|s| s.parse::<i64>()).transpose()?,
        discord_id: metadata_public.discord_id.map(|s| s.parse::<i64>()).transpose()?,
        google_id:  metadata_public.google_id.map(|s| s.parse::<i128>()).transpose()?,
        gitlab_id: metadata_public.gitlab_id.map(|s| s.parse::<i64>()).transpose()?,
        microsoft_id:  metadata_public.microsoft_id.map(|s| s.parse::<i64>()).transpose()?,
        apple_id: metadata_public.apple_id.map(|s| s.parse::<i64>()).transpose()?,
    };
    Ok(HttpResponse::Ok().json(minos_user))
}
