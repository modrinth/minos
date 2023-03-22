

use crate::routes::ApiError;
use actix_web::{web, HttpResponse, post};
use ory_client::models;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ImportedUser {
    pub schema_id : String,
    pub traits : ImportedUserTraits,
    pub credentials : CredentialsType
}
#[derive(Serialize, Debug)]
pub struct ImportedUserTraits {
    pub email : String
}

#[derive(Serialize, Debug)]
pub enum CredentialsType {
    Password(models::IdentityCredentials),
    Oidc(models::IdentityCredentials)
}
impl ImportedUser {
    // Create a user importable to Ory Kratos using the 'password' method
    // 'hashed_password' must be hashed using a recognized hashing method (ie: bcrypt)
    fn build_password(email : String, hashed_password : String) -> ImportedUser {
        let config = Some(
            serde_json::to_value(
                models::IdentityCredentialsPassword {
                    hashed_password: Some(hashed_password)
                }
            ).unwrap()
        );
        ImportedUser { 
            schema_id: "default".to_string(), 
            traits: ImportedUserTraits {
                 email 
            },
            credentials: CredentialsType::Password(
                models::IdentityCredentials {
                    config,
                    created_at: None,
                    identifiers: None,
                    updated_at: None,
                    _type: None,
                    version: None,

            })
        }
    }
    // Create a user importable to Ory Kratos using the 'oidc' method
    // Provider is 'github', 'discord' or other supported oidc method, and subject is that users name within the OIDC
    fn build_oidc(email : String, provider_subjects : Vec<(String, String)>) -> ImportedUser {
        let providers = provider_subjects.into_iter().map(
            |(provider, subject)| models::IdentityCredentialsOidcProvider {
                provider : Some(provider),
                subject: Some(subject),
                initial_access_token: None,
                initial_id_token: None,
                initial_refresh_token: None,
        }).collect();
        let config = Some(
            serde_json::to_value(
                models::IdentityCredentialsOidc {
                    providers: Some(providers)
                }
            ).unwrap()
        );
        ImportedUser { 
            schema_id: "default".to_string(), 
            traits: ImportedUserTraits {
                 email 
            },
            credentials: CredentialsType::Oidc(
                models::IdentityCredentials {
                    config,
                    created_at: None,
                    identifiers: None,
                    updated_at: None,
                    _type: None,
                    version: None,
            })
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum NewUserData {
    NewUserDataOidc(UserDataOidc),
    NewUserDataPassword(UserDataPassword)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserDataOidc {
    pub email : String,
    pub provider: String,
    pub subject: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserDataPassword {
    pub email : String,
    pub hashed_password: String,
}

// Add account manually
// Input is body matching:
/*
    Add account manually
    Input is body matching:
    {
        email: "",
        provider: "",
        subject: "",
    }
    or
    {
        email: "",
        hashed_password: "",
    }
    Provider is 'github', 'discord' or other supported oidc method, and subject is that users name within the OIDC
    'hashed_password' must be hashed using a recognized hashing method (ie: bcrypt)
 */
#[post("import_account")]
pub async fn import_account(data : web::Json<NewUserData>) -> Result<HttpResponse, ApiError> {
    let data = data.into_inner();
    println!("Start");
    dbg!(&data);

    // Create importable user in required Ory Kratos format
    let imported_user = match data {
        NewUserData::NewUserDataOidc(user) => 
            ImportedUser::build_oidc(user.email, vec![(user.provider, user.subject)]),
        NewUserData::NewUserDataPassword(user) => 
            ImportedUser::build_password(user.email, user.hashed_password),
    };

    // Create request to Kratos admin API
    let client = reqwest::Client::new();
    let ory_admin_bearer = format!("Bearer {}", dotenvy::var("ORY_AUTH_BEARER").unwrap());
    let identities_route = format!("{}/admin/identities", dotenvy::var("ORY_URL").unwrap());
    let res = client
        .post(&identities_route)
        .header("Authorization", &ory_admin_bearer)
        .header("Accept", "*/*")
        .json(&imported_user)
        .send()
        .await?
        .text().await?;
    Ok(HttpResponse::Ok().body(res))
}

