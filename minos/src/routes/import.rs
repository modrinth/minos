
use crate::database::models::labrinth_user::LabrinthUser;
use crate::routes::ApiError;
use actix_web::{post, web, HttpResponse};
use futures::future::try_join_all;
use ory_client::apis::identity_api::create_identity;
use ory_client::models::{self, CreateIdentityBody, Identity};
use ory_client::{self, apis::configuration::Configuration};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::OryError;
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ImportUsers {
    NewUser(NewUserData),
    NewUsers(Vec<NewUserData>),
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

#[derive(Serialize, Debug)]
pub struct ImportedUserTraits<'a> {
    pub email: &'a str,
}


// POST /admin/import_account
// Requires admin bearer token as header.
/*
    Add account manually
    Input is *one account* or an *array* of accounts, where each account has body matching:
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
    data: web::Json<ImportUsers>,
) -> Result<HttpResponse, ApiError> {
    let data = data.into_inner();
    match data {
        ImportUsers::NewUser(user) => {
            let res = import_account_helper(&user, &configuration).await?;
            Ok(HttpResponse::Ok().json(res))
        }
        ImportUsers::NewUsers(users) => {
            let res = users
                .iter()
                .map(|u| import_account_helper(u, &configuration));
            let res = try_join_all(res).await?;
            Ok(HttpResponse::Ok().json(res))
        }
    }
}

async fn import_account_helper(
    data: &NewUserData,
    configuration: &Configuration,
) -> Result<Identity, ApiError> {
    // Create importable user in required Ory Kratos format
    let create_identity_body = match data {
        NewUserData::NewUserDataOidc(user) => {
            build_oidc(&user.email, vec![(&user.provider, &user.subject)])
        }
        NewUserData::NewUserDataPassword(user) => {
            build_password(&user.email, &user.hashed_password)
        }
    }?;
    let res = create_identity(configuration, Some(&create_identity_body))
        .await
        .map_err(OryError::from)?;
    Ok(res)
}

fn build_password(email: &str, hashed_password: &str) -> Result<CreateIdentityBody, OryError> {
    let mut create_identity_body = CreateIdentityBody::new(
        "default".to_string(),
        serde_json::to_value(ImportedUserTraits { email })?,
    );
    // Create credentials and add it to identity body
    let credentials_pass_config = models::IdentityWithCredentialsPasswordConfig {
        hashed_password: Some(hashed_password.to_string()),
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
    email: &str,
    provider_subjects: Vec<(&str, &str)>,
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
                provider: provider.to_string(),
                subject: subject.to_string(),
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


// POST pull_labrinth
// Requires admin bearer token as header.
#[post("pull_labrinth")]
pub async fn pull_labrinth_github_accounts(
    configuration: web::Data<Configuration>,
    pool: web::Data<PgPool>,

) -> Result<HttpResponse, ApiError> {
    let _client = reqwest::Client::new();
    
    let users = LabrinthUser::get_all(        &**pool, ).await?;
    let users : Vec<_> = users.into_iter().filter_map(|lu| {
        Some(NewUserData::NewUserDataOidc(UserDataOidc {
            email: lu.email?,
            provider: "github".to_string(),
            subject: lu.github_id?.to_string(),
        }))
    }).collect();
    
    let res = users.iter().map(|f| import_account_helper(f, &configuration));
    let res = try_join_all(res).await?;
    Ok(HttpResponse::Ok().json(res))
}