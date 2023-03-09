use crate::error::ApiError;
use actix_web::{HttpResponse, Responder};

// Default handling for any non-existent route
pub async fn not_found() -> impl Responder {
    let data = ApiError {
        error: "not_found",
        description: "the requested route does not exist",
    };
    HttpResponse::NotFound().json(data)
}
