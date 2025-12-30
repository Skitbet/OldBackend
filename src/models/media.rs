use serde::{ Serialize, Deserialize };
use chrono::{ DateTime, Utc };
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Post,
    Character,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Media {
    pub url: String,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub uploaded_at: DateTime<Utc>,
    pub is_nsfw: Option<bool>,

    pub metadata: MediaMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", content = "data")]
pub enum MediaMetadata {
    Post {
        width: Option<u32>,
        height: Option<u32>,
        duration_secs: Option<f32>,
    },
    Character {
        character_id: String,
        pose: Option<String>,
        emotion: Option<String>,
        notes: Option<String>,
    },
    Other {
        description: Option<String>,
    },
}
