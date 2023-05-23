// Return structure for webhooks
#[derive(Debug, Deserialize, Serialize)]
pub struct OryWebhookPayload {
    pub messages: Vec<OryWebhookMessagePacket>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OryWebhookMessagePacket {
    pub instance_ptr: String,
    pub messages: Vec<OryMessage>,
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OryMessage {
    pub id: i32,
    pub text: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<HashMap<String, String>>,
}
