use std::collections::HashMap;

use crate::database::models::webhook::OryMessage;
use crate::database::models::webhook::OryWebhookMessagePacket;
use crate::database::models::webhook::OryWebhookPayload;
use crate::routes::ApiError;
use actix_web::ResponseError;

// An error type that can be returned to the Ory system as a webhook response
// Errors in this format can be parsed by the Ory system and displayed to the user elegantly
#[derive(Debug)]
pub struct CallbackError {
    pub name: String,
    pub id: i32,
    pub text: String,
    pub r#type: String,
    pub status_code: actix_web::http::StatusCode,
}

impl actix_web::ResponseError for CallbackError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.status_code
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        get_error_response(&self)
    }
}

fn get_error_response(e : &CallbackError) -> actix_web::HttpResponse {
    actix_web::HttpResponse::build(e.status_code()).json(OryWebhookPayload {
        messages: vec![OryWebhookMessagePacket {
            instance_ptr: e.name.to_string(),
            messages: vec![OryMessage {
                id: e.id,
                text: e.text.to_string(),
                r#type: e.r#type.to_string(),
                context: Some(HashMap::new()),
            }],
        }],
    })
}

impl std::fmt::Display for CallbackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CallbackError: {}", self.text)
    }
}

// we impl From from ApiError, creating a CallbackError with default members
impl From<ApiError> for CallbackError {
    fn from(err: ApiError) -> Self {
        CallbackError {
            name: "#/traits/email".to_string(),
            id: 1,
            text: err.to_string(),
            r#type: err.get_error_response().to_string(),
            status_code: err.status_code(),
        }
    }
}
