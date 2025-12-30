use std::collections::HashSet;

use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::media::Media;
use crate::utils::post::PostType;
use crate::utils::uuid_as_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub id: Uuid,
    pub author: String,
    #[serde(with = "uuid_as_string")]
    pub author_id: Uuid,
    pub title: String,
    pub body: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub post_type: PostType,
    pub nsfw: bool,
    #[serde(default)]
    pub likes: HashSet<String>,
    #[serde(default)]
    pub dislikes: HashSet<String>,
    pub media: Vec<Media>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AdminPatchPost {
    pub author: Option<String>,
    pub author_id: Option<Uuid>,
    pub title: Option<String>,
    pub body: Option<Option<String>>,
    pub tags: Option<Vec<String>>,
    pub post_type: Option<PostType>,
    pub nsfw: Option<bool>,
    pub likes: Option<HashSet<String>>,
    pub dislikes: Option<HashSet<String>>,
    // pub media_urls: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PostResponse {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub id: Uuid,
    pub short_id: String,
    pub author: String,
    pub author_id: Uuid,
    pub title: String,
    pub body: Option<String>,
    pub tags: Vec<String>,
    pub post_type: PostType,
    pub nsfw: bool,
    pub likes: HashSet<String>,
    pub dislikes: HashSet<String>,
    pub media: Vec<Media>,
    #[serde(serialize_with = "chrono::serde::ts_milliseconds::serialize")]
    pub created_at: DateTime<Utc>,
}

impl From<&Post> for PostResponse {
    fn from(value: &Post) -> Self {
        PostResponse {
            id: value.id,
            short_id: value.id.to_string()[..8].to_string(),
            author: value.author.clone(),
            author_id: value.author_id,
            title: value.title.clone(),
            body: value.body.clone(),
            tags: value.tags.clone(),
            post_type: value.post_type.clone(),
            nsfw: value.nsfw,
            likes: value.likes.clone(),
            dislikes: value.dislikes.clone(),
            media: value.media.clone(),
            created_at: value.created_at,
        }
    }
}

impl Post {
    pub fn apply_patch(&mut self, patch: AdminPatchPost) {
        if let Some(author) = patch.author {
            self.author = author;
        }
        if let Some(author_id) = patch.author_id {
            self.author_id = author_id;
        }
        if let Some(title) = patch.title {
            self.title = title;
        }
        if let Some(body) = patch.body {
            self.body = body;
        }
        if let Some(tags) = patch.tags {
            self.tags = tags;
        }
        if let Some(post_type) = patch.post_type {
            self.post_type = post_type;
        }
        if let Some(nsfw) = patch.nsfw {
            self.nsfw = nsfw;
        }
        if let Some(likes) = patch.likes {
            self.likes = likes;
        }
        if let Some(dislikes) = patch.dislikes {
            self.dislikes = dislikes;
        }
        // if let Some(media_urls) = patch.media_urls {
        //     self.media_urls = media_urls;
        // }
        if let Some(created_at) = patch.created_at {
            self.created_at = created_at;
        }
    }
}
