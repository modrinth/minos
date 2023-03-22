use crate::routes::ApiError;
use actix_web::{post, web, HttpResponse};
use ory_client::apis::identity_api::create_identity;
use ory_client::models::{self, CreateIdentityBody};
use ory_client::{self, apis::configuration::Configuration};
use serde::{Deserialize, Serialize};

use super::OryError;

#[derive(Serialize, Debug)]
pub struct ImportedUserTraits {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum NewUserData {
    NewUserDataOidc(UserDataOidc),
    NewUserDataPassword(UserDataPassword),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserDataOidc {
    pub email: String,
    pub provider: String,
    pub subject: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserDataPassword {
    pub email: String,
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
pub async fn import_account(
    configuration: web::Data<Configuration>,
    data: web::Json<NewUserData>,
) -> Result<HttpResponse, ApiError> {
    let data = data.into_inner();

    // Create importable user in required Ory Kratos format
    let create_identity_body = match data {
        NewUserData::NewUserDataOidc(user) => {
            build_oidc(user.email, vec![(user.provider, user.subject)])
        }
        NewUserData::NewUserDataPassword(user) => build_password(user.email, user.hashed_password),
    }?;
    let res = create_identity(&configuration, Some(&create_identity_body))
        .await
        .map_err(|e| OryError::from(e))?;
    Ok(HttpResponse::Ok().json(res))
}

fn build_password(email: String, hashed_password: String) -> Result<CreateIdentityBody, OryError> {
    let mut create_identity_body = CreateIdentityBody::new(
        "default".to_string(),
        serde_json::to_value(ImportedUserTraits { email })?,
    );
    // Create credentials and add it to identity body
    let credentials_pass_config = models::IdentityWithCredentialsPasswordConfig {
        hashed_password: Some(hashed_password),
        password: None,
    };
    let credentials_pass = models::IdentityWithCredentialsPassword {
        config: Some(Box::new(credentials_pass_config)),
    };

    let credentials = models::IdentityWithCredentials {
        password: Some(Box::new(credentials_pass)),
        oidc: None,
    };
    create_identity_body.credentials = Some(Box::new(credentials));

    Ok(create_identity_body)
}

fn build_oidc(
    email: String,
    provider_subjects: Vec<(String, String)>,
) -> Result<CreateIdentityBody, OryError> {
    let mut create_identity_body = CreateIdentityBody::new(
        "default".to_string(),
        serde_json::to_value(ImportedUserTraits { email })?,
    );

    // Create credentials and add it to identity body
    let providers = provider_subjects
        .into_iter()
        .map(
            |(provider, subject)| models::IdentityWithCredentialsOidcConfigProvider {
                provider,
                subject,
            },
        )
        .collect();
    let credentials_oidc_config = models::IdentityWithCredentialsOidcConfig {
        config: None,
        providers: Some(providers),
    };
    let credentials_oidc = models::IdentityWithCredentialsOidc {
        config: Some(Box::new(credentials_oidc_config)),
    };
    let credentials = models::IdentityWithCredentials {
        password: None,
        oidc: Some(Box::new(credentials_oidc)),
    };
    create_identity_body.credentials = Some(Box::new(credentials));

    Ok(create_identity_body)
}
