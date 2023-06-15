use ory_client::models::Identity;
use serde::{Deserialize, Serialize};

use crate::routes::{user::MinosSessionTraits, ApiError, OryError};

#[derive(Deserialize, Serialize)]
pub struct EmailPayload {
    email: String,
}

// Updates an email back to labrinth, for its local storage (So labrinth does not need to call Minos on its many email-related checks)
pub async fn email_update(
    identity_with_credentials: &Identity,
) -> Result<actix_web::HttpResponse, ApiError> {
    // Get the traits with email
    let traits = identity_with_credentials
        .traits
        .clone()
        .ok_or_else(|| OryError::MissingIdentityData("traits".to_string()))?;

    let traits = serde_json::from_value::<MinosSessionTraits>(traits)?;
    let email = traits.email.clone();
    let kratos_id = &identity_with_credentials.id;

    let client = reqwest::Client::new();
    let _res = client
        .post(format!(
            "{}/admin/_edit_email/{kratos_id}",
            dotenvy::var("LABRINTH_API_URL").unwrap()
        ))
        .header("Accept", "application/json")
        .header("Accept-Language", "en_US")
        .header(
            "Modrinth-Admin",
            dotenvy::var("LABRINTH_ADMIN_KEY").unwrap(),
        )
        .header(
            "x-ratelimit-key",
            dotenvy::var("RATE_LIMIT_IGNORE_KEY").unwrap(),
        )
        .json(&EmailPayload { email })
        .send()
        .await?;
    Ok(actix_web::HttpResponse::Ok().json(traits.email))
}
