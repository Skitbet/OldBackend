use actix_web::{get, post, web::{self, Data, Path}, HttpResponse, Responder};
use uuid::Uuid;
use std::str::FromStr;

use crate::{
    middleware::auther::Auther,
    models::comment::{Comment, CommentReqInput, Reply},
    state::AppState,
    utils::error::AppError,
};
use crate::routes::internal::comment::utils::toggle_reaction;
use super::types::*;
#[get("/fetch/{id}")]
pub async fn get_comments(
    path: Path<String>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let post_id = path.into_inner();

    let comments = state.db.comments.get_for_post(&post_id).await?;

    let mut result = Vec::new();
    for comment in comments {
        let replies_doc = state.db.comment_replies.get_replies(&comment.id.to_string()).await;
        let has_replies = replies_doc.map(|r| r.has_replies).unwrap_or(false);

        result.push(CommentWithRepliesFlag {
            comment,
            has_replies,
        });
    }

    Ok(HttpResponse::Ok().json(result))
}

#[post("/create/{id}")]
pub async fn post_comment(
    _author: Auther,
    path: Path<String>,
    data: web::Json<CommentReqInput>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let post_id = Uuid::from_str(&path.into_inner())
        .map_err(|_| AppError::BadRequest("Invalid post_id".into()))?;

    let comment = Comment::new(data.author.clone(), data.content.clone(), post_id);
    state.db.comments.create(&comment).await?;
    state.db.comment_replies.create(&comment.id).await?;
    Ok(HttpResponse::Ok().json(comment))
}

#[post("/{id}/like")]
pub async fn like_comment(
    author: Auther,
    path: Path<String>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let session = author.session;
    let user_id = session.user_uuid.to_string();
    let mut comment = state.db.comments.get_by_id(&path.into_inner()).await?;

    let liked = toggle_reaction(&mut comment.likes, &mut comment.dislikes, &user_id, true);

    state.db.comments.save(&comment).await?;
    Ok(HttpResponse::Ok().json(ToggleResponse { liked }))
}

#[post("/{id}/dislike")]
pub async fn dislike_comment(
    author: Auther,
    path: Path<String>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let session = author.session;
    let user_id = session.user_uuid.to_string();
    let mut comment = state.db.comments.get_by_id(&path.into_inner()).await?;

    let disliked = toggle_reaction(&mut comment.likes, &mut comment.dislikes, &user_id, false);

    state.db.comments.save(&comment).await?;
    Ok(HttpResponse::Ok().json(DislikeResponse { disliked }))
}

#[post("/reply/{parent_id}")]
pub async fn reply_to_comment(
    author: Auther,
    path: Path<String>,
    state: Data<AppState>,
    data: web::Json<CommentReplyInput>,
) -> Result<impl Responder, AppError> {
    let  parent_id = path.into_inner();

    let session = author.session;
    let user_id = session.user_uuid.to_string();
    let user = state.db.users.get_by_uuid(&Uuid::from_str(&*user_id).unwrap()).await?;

    let reply = Reply::new(user.username, data.content.clone());
    state.db.comment_replies.add_reply_by_parent_id(&parent_id, &reply).await?;

    Ok(HttpResponse::Ok().json(reply))
}

#[get("/reply/{comment_id}")]
pub async fn get_replies(
    path: Path<String>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let replies = state.db.comment_replies.get_replies(&path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(replies))
}

#[post("/reply/{id}/like")]
pub async fn like_reply(
    author: Auther,
    path: Path<String>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let reply_id = path.into_inner();
    let session = author.session;
    let user_id = session.user_uuid.to_string();

    let updated = state.db.comment_replies.like_reply(&reply_id, &user_id).await?;
    Ok(HttpResponse::Ok().json(ToggleResponse { liked: updated }))
}

#[post("/reply/{id}/dislike")]
pub async fn dislike_reply(
    author: Auther,
    path: Path<String>,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let reply_id = path.into_inner();
    let session = author.session;
    let user_id = session.user_uuid.to_string();

    let updated = state.db.comment_replies.dislike_reply(&reply_id, &user_id).await?;
    Ok(HttpResponse::Ok().json(DislikeResponse { disliked: updated }))
}
