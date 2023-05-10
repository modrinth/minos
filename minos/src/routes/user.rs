use crate::routes::ApiError;
use actix_web::{get, web, HttpResponse};
use ory_client::models::Session;
use serde::{Serialize, Deserialize};

// GET /user/session
// Returns 200, and returns the authenticated session.
// This is encapsuled by the authentication middleware (returning 401 in middleware if it fails to authenticate)
#[get("session")]
pub async fn user_session_get(session: Option<web::ReqData<Session>>) -> Result<HttpResponse, ApiError> {
    println!("Inside session!");
    let session = session.ok_or(ApiError::SessionError)?;
    Ok(HttpResponse::Ok().json(&*session))
}

/*
    Relevant Minos Session JSON example:
    {
    "active": true,
    "authenticated_at": "2023-05-10T00:11:02.442489895Z",
    "authentication_methods": [
        {
        "aal": "aal1",
        "completed_at": "2023-05-10T00:11:02.407307111Z",
        "method": "oidc"
        }
    ],
    "authenticator_assurance_level": "aal1",
    "devices": [
        {
        "id": "e7a19901-23a6-4924-89d2-05e264f5ba2b",
        "ip_address": "192.168.208.1:54962",
        "location": "",
        "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.35"
        }
    ],
    "expires_at": "2023-05-11T00:11:02.407276861Z",
    "id": "4ab8afc4-25e0-46c8-8fdb-1d3d86bd41bd",
    "identity": {
        "created_at": "2023-05-10T00:11:02.391292Z",
        "id": "8e1b56c4-80fe-4d6e-8251-61f6657a554a",
        "recovery_addresses": [
        {
            "created_at": "2023-05-10T00:11:02.394114Z",
            "id": "4cde2894-4a3b-4295-9440-f4d4ff4c0c9a",
            "updated_at": "2023-05-10T00:11:02.394114Z",
            "value": "thesuzerain1@gmail.com",
            "via": "email"
        }
        ],
        "schema_id": "default",
        "schema_url": "http://127.0.0.1:4433/schemas/ZGVmYXVsdA",
        "state": "active",
        "state_changed_at": "2023-05-10T00:11:02.390487328Z",
        "traits": {
        "email": "thesuzerain1@gmail.com",
        "username": "thesuzerain"
        },
        "updated_at": "2023-05-10T00:11:02.391292Z",
        "verifiable_addresses": [
        {
            "created_at": "2023-05-10T00:11:02.393118Z",
            "id": "6004b058-0f7d-40a5-b5b9-4bc10e826b3b",
            "status": "sent",
            "updated_at": "2023-05-10T00:11:02.393118Z",
            "value": "thesuzerain1@gmail.com",
            "verified": false,
            "via": "email"
        }
        ]
    },
    "issued_at": "2023-05-10T00:11:02.407276861Z"
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
    pub github_id: Option<i64>,
    pub discord_id: Option<String>,
    pub google_id: Option<String>,
    pub gitlab_id: Option<String>,
    pub microsoft_id: Option<String>,
    pub apple_id: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinosUser {
    pub id: String, // This is the unique generated Ory name
    pub username: String,  // unique username
    pub email: String,
    pub name: Option<String>, // real name
    pub github_id: Option<i64>,
    pub discord_id: Option<String>,
    pub google_id: Option<String>,
    pub gitlab_id: Option<String>,
    pub microsoft_id: Option<String>,
    pub apple_id: Option<String>
}

#[get("")]
pub async fn user_get(session: Option<web::ReqData<Session>>) -> Result<HttpResponse, ApiError> {
    let session = session.ok_or(ApiError::SessionError)?;
    let identity = &session.identity;
    let metadata_public: MinosSessionMetadataPublic = if let Some(m) = identity.metadata_public.clone() {serde_json::from_value(m)?} else { Default::default()};
    let traits: MinosSessionTraits = serde_json::from_value(identity.traits.as_ref().ok_or_else(|| ApiError::SessionError)?.clone())?;

    let minos_user = MinosUser {
        id: identity.id.clone(),
        username: traits.username,
        email: traits.email,
        name: None,
        github_id: metadata_public.github_id,
        discord_id: metadata_public.discord_id,
        google_id: metadata_public.google_id,
        gitlab_id: metadata_public.gitlab_id,
        microsoft_id: metadata_public.microsoft_id,
        apple_id: metadata_public.apple_id
    };
    Ok(HttpResponse::Ok().json(minos_user))
}
