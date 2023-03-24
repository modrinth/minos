use actix_web::{delete, web, HttpResponse};
use futures::future::try_join_all;
use ory_client::apis::configuration::Configuration;
use ory_client::apis::identity_api;
use ory_client::models::Identity;

use crate::routes::{ApiError, OryError};

// DELETE /admin/delete_all
// Requires admin bearer token as header.
// TEMPORARY ENDPOINT. TODO: DELETE THIS.
// A temporary endpoint that deletes all users.
// This should *only* be used for testing purposes.
#[delete("delete_all")]
pub async fn delete_all(configuration: web::Data<Configuration>) -> Result<HttpResponse, ApiError> {
    // Fetch all identities
    let identities: Vec<Identity> = identity_api::list_identities(&configuration, None, None, None)
        .await
        .map_err(|e| OryError::from(e))?;

    // Delete identities (using concurrency)
    let identity_futures = identities
        .iter()
        .map(|i| identity_api::delete_identity(&configuration, &i.id));
    try_join_all(identity_futures)
        .await
        .map_err(|e| OryError::from(e))?;
    Ok(HttpResponse::Ok().json(identities))
}
