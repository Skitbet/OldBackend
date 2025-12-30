use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use crate::models::comment::Comment;
#[derive(Deserialize, Clone, ToSchema)]
pub struct CommentReplyInput {
    pub content: String,
}

#[derive(Serialize, ToSchema)]
pub struct CommentWithRepliesFlag {
    #[serde(flatten)]
    pub comment: Comment,
    pub has_replies: bool,
}

#[derive(Serialize, ToSchema)]
pub struct ToggleResponse {
    pub liked: bool,
}

#[derive(Serialize, ToSchema)]
pub struct DislikeResponse {
    pub disliked: bool,
}
