use actix_web::web;
use log::info;
use crate::routes::internal::comment::handlers::{
    dislike_comment,
    dislike_reply,
    get_comments,
    get_replies,
    like_comment,
    like_reply,
    post_comment,
    reply_to_comment,
};

mod handlers;
pub mod types;
mod utils;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring /api/comment scope");
    cfg.service(
        web
            ::scope("comment")
            .service(like_comment)
            .service(dislike_comment)
            .service(get_comments)
            .service(post_comment)
            .service(reply_to_comment)
            .service(get_replies)
            .service(like_reply)
            .service(dislike_reply)
    );
}
