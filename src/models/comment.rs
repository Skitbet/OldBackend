use std::collections::HashSet;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use uuid::Uuid;

use crate::utils::uuid_as_string;

#[derive(Deserialize, Clone, ToSchema)]
pub struct CommentReqInput {
    pub author: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Comment {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub id: Uuid,
    #[serde(with = "uuid_as_string")]
    pub post_id: Uuid, // id of the post the comment is linked to
    pub author: String,
    pub content: String,
    pub likes: HashSet<String>,
    pub dislikes: HashSet<String>,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Reply {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub id: Uuid,
    pub author: String,
    pub content: String,
    pub likes: HashSet<String>,
    pub dislikes: HashSet<String>,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
    #[schema(value_type = Vec<String>, example = json!(["child_reply_id_1", "child_reply_id_2"]))]
    pub replies: Vec<Reply>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CommentReplies {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub id: Uuid,
    pub has_replies: bool,
    pub sub_ids: HashSet<String>,
    pub replies: Vec<Reply>,
}

impl Comment {
    pub fn new(author_username: String, content: String, post_id: Uuid) -> Self {
        let now = Utc::now();
        Comment {
            id: Uuid::new_v4(),
            post_id,
            author: author_username,
            content,
            likes: HashSet::new(),
            dislikes: HashSet::new(),
            created_at: now,
            edited_at: None,
        }
    }
}

impl Reply {
    pub fn new(author_username: String, content: String) -> Self {
        let now = Utc::now();
        Reply {
            id: Uuid::new_v4(),
            author: author_username,
            content,
            likes: HashSet::new(),
            dislikes: HashSet::new(),
            created_at: now,
            edited_at: None,
            replies: vec![],
        }
    }
}
