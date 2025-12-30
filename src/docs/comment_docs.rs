#![allow(dead_code)]
use crate::{
    models::comment::{ Comment, CommentReqInput, Reply },
    routes::internal::comment::types::{
        CommentReplyInput,
        CommentWithRepliesFlag,
        DislikeResponse,
        ToggleResponse,
    },
};

#[utoipa::path(
    get,
    path = "/api/comments/fetch/{id}",
    params(("id" = String, Path, description = "Post ID to fetch comments for")),
    responses(
        (status = 200, description = "List of comments for post", body = [CommentWithRepliesFlag]),
        (status = 404, description = "Post not found")
    ),
    tag = "Comments"
)]
pub async fn get_comments() {}

#[utoipa::path(
    post,
    path = "/api/comments/create/{id}",
    request_body = CommentReqInput,
    params(("id" = String, Path, description = "Post ID to comment on")),
    params(("Authorization" = String, Header, description = "Bearer token for user.")),
    responses(
        (status = 200, description = "Comment created successfully", body = Comment),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Comments"
)]
pub async fn post_comment() {}

#[utoipa::path(
    post,
    path = "/api/comments/{id}/like",
    params(("id" = String, Path, description = "Comment ID to like")),
    params(("Authorization" = String, Header, description = "Bearer token for user.")),
    responses(
        (status = 200, description = "Toggled like state", body = ToggleResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Comment not found")
    ),
    tag = "Comments"
)]
pub async fn like_comment() {}

#[utoipa::path(
    post,
    path = "/api/comments/{id}/dislike",
    params(("id" = String, Path, description = "Comment ID to dislike")),
    params(("Authorization" = String, Header, description = "Bearer token for user.")),
    responses(
        (status = 200, description = "Toggled dislike state", body = DislikeResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Comment not found")
    ),
    tag = "Comments"
)]
pub async fn dislike_comment() {}

#[utoipa::path(
    post,
    path = "/api/comments/reply/{parent_id}",
    params(("parent_id" = String, Path, description = "Parent comment ID to reply to")),
    request_body = CommentReplyInput,
    params(("Authorization" = String, Header, description = "Bearer token for user.")),
    responses(
        (status = 200, description = "Reply added successfully", body = Reply),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Parent comment not found")
    ),
    tag = "Comments"
)]
pub async fn reply_to_comment() {}

#[utoipa::path(
    get,
    path = "/api/comments/reply/{comment_id}",
    params(("comment_id" = String, Path, description = "Comment ID to fetch replies for")),
    responses(
        (status = 200, description = "List of replies for comment", body = [Reply]),
        (status = 404, description = "Comment not found")
    ),
    tag = "Comments"
)]
pub async fn get_replies() {}

#[utoipa::path(
    post,
    path = "/api/comments/reply/{id}/like",
    params(("id" = String, Path, description = "Reply ID to like")),
    params(("Authorization" = String, Header, description = "Bearer token for user.")),
    responses(
        (status = 200, description = "Toggled like state on reply", body = ToggleResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Reply not found")
    ),
    tag = "Comments"
)]
pub async fn like_reply() {}

#[utoipa::path(
    post,
    path = "/api/comments/reply/{id}/dislike",
    params(("id" = String, Path, description = "Reply ID to dislike")),
    params(("Authorization" = String, Header, description = "Bearer token for user.")),
    responses(
        (status = 200, description = "Toggled dislike state on reply", body = DislikeResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Reply not found")
    ),
    tag = "Comments"
)]
pub async fn dislike_reply() {}
