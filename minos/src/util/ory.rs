use ory_client::apis::configuration::Configuration;
use reqwest::Client;

#[derive(Clone)]
pub struct BasicConfiguration(pub Configuration);

#[derive(Clone)]
pub struct AdminConfiguration(pub Configuration);

pub fn generate_basic_configuration() -> BasicConfiguration {
    // Default Ory configuration
    let configuration = Configuration {
        api_key: None,
        base_path: dotenvy::var("ORY_URL").unwrap(),
        client: Client::new(),
        basic_auth: None,
        user_agent: Some("Modrinth Minos".to_string()),
        oauth_access_token: None,
        bearer_access_token: None,
    };
    BasicConfiguration(configuration)
}

pub fn generate_admin_configuration() -> AdminConfiguration {
    // Default Ory configuration
    let configuration = Configuration {
        api_key: None,
        base_path: dotenvy::var("ORY_ADMIN_URL").unwrap(),
        client: Client::new(),
        basic_auth: None,
        user_agent: Some("Modrinth Minos Admin".to_string()),
        oauth_access_token: None,
        bearer_access_token: None,
    };
    AdminConfiguration(configuration)
}
