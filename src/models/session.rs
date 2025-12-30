use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::uuid_as_string;

/// Represents a logged-in session for a user.
///
/// - `token`: The JWT string.
/// - `session_id`: Unique UUID for tracking this session.
/// - `user_uuid`: Link to the owning user.
/// - `created_at`: When the session was issued.
/// - `expires_at`: When the session will expire.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    /// MongoDB object ID
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub token: String,
    #[serde(with = "uuid_as_string")]
    pub session_id: Uuid,
    #[serde(with = "uuid_as_string")]
    pub user_uuid: Uuid,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: chrono::DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub expires_at: chrono::DateTime<Utc>,
    // pub ip_address: Option<String>,
}
