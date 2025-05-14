use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ActivationToken {
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}
