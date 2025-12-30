use std::collections::HashSet;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use crate::{ models::media::Media, utils::uuid_as_string };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub id: Uuid,
    pub owner: String,
    #[serde(with = "uuid_as_string")]
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub likes: HashSet<String>,
    #[serde(default)]
    pub dislikes: HashSet<String>,
    pub media: Vec<Media>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
}
