use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LabrinthUser {
    pub id: String,
    pub github_id: Option<i64>,
}
