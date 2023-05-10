use super::OryError;
use crate::routes::ApiError;
use actix_web::{post, web, HttpResponse};
use futures::future::try_join_all;
use ory_client::apis::identity_api::create_identity;
use ory_client::models::{self, CreateIdentityBody, Identity};
use ory_client::{self, apis::configuration::Configuration};
use serde::{Deserialize, Serialize};
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
    pub username: String,
    pub provider: String,
    pub subject: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserDataPassword {
    pub email: String,
    pub username: String,
    pub hashed_password: String,
}

#[derive(Serialize, Debug)]
pub struct ImportedUserTraits<'a> {
    pub email: &'a str,
    pub username: &'a str,
}

// POST /admin/import_account
// Requires admin bearer token as header.
/*
    Add account manually
    Input is *one account* or an *array* of accounts, where each account has body matching:
   {
       email: "",
       username: "",
       provider: "",
       subject: "",
   }
   or
   {
       email: "",
       username: "",
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
        NewUserData::NewUserDataOidc(user) => build_oidc(
            &user.email,
            &user.username,
            vec![(&user.provider, &user.subject)],
        ),
        NewUserData::NewUserDataPassword(user) => {
            build_password(&user.email, &user.username, &user.hashed_password)
        }
    }?;
    let res = create_identity(configuration, Some(&create_identity_body))
        .await
        .map_err(OryError::from)?;
    Ok(res)
}

fn build_password(
    email: &str,
    username: &str,
    hashed_password: &str,
) -> Result<CreateIdentityBody, OryError> {
    let mut create_identity_body = CreateIdentityBody::new(
        "default".to_string(),
        serde_json::to_value(ImportedUserTraits { email, username })?,
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
    username: &str,
    provider_subjects: Vec<(&str, &str)>,
) -> Result<CreateIdentityBody, OryError> {
    let mut create_identity_body = CreateIdentityBody::new(
        "default".to_string(),
        serde_json::to_value(ImportedUserTraits { email, username })?,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LabrinthUser {
    pub id: i64,
    pub github_id: Option<i64>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}

// POST pull_labrinth
// Requires admin bearer token as header.
#[post("pull_labrinth")]
pub async fn pull_labrinth_github_accounts(
    configuration: web::Data<Configuration>,
) -> Result<HttpResponse, ApiError> {
    let client = reqwest::Client::new();

    let users: Vec<LabrinthUser> = client
        .get(format!("{}minos/export", dotenvy::var("LABRINTH_API_URL")?))
        .header("Accept", "application/json")
        .header("Accept-Language", "en_US")
        .send()
        .await?
        .json()
        .await?;

    // Convert to ory format and import
    let users: Vec<NewUserData> = users
        .into_iter()
        .filter_map(|u| {
            Some(NewUserData::NewUserDataOidc(UserDataOidc {
                email: u.email?,
                username: u.username?,
                provider: "github".to_string(),
                subject: u.github_id?.to_string(),
            }))
        })
        .collect();

    // TODO: make these import asynchronously
    let res = users
        .iter()
        .map(|f| import_account_helper(f, &configuration));
    let res = try_join_all(res).await?;
    Ok(HttpResponse::Ok().json(res))
}
