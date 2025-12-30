use actix_web::web;
use log::info;
use crate::routes::internal::posts::handler::{
    create_post,
    delete_post,
    dislike_post,
    edit_post,
    get_a_random_post,
    get_latest_posts,
    get_popular_posts,
    get_post,
    get_post_by_id,
    get_premium_posts,
    get_random_posts,
    like_post,
    search_posts,
};

pub mod handler;
mod upload;
pub mod types;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring /api/posts scope");
    cfg.service(
        web
            ::scope("posts")
            .service(search_posts)
            .service(edit_post)
            .service(create_post)
            .service(get_latest_posts)
            .service(get_post)
            .service(like_post)
            .service(dislike_post)
            .service(delete_post)
            .service(get_post_by_id)
            .service(get_popular_posts)
            .service(get_premium_posts)
            .service(get_random_posts)
            .service(get_a_random_post)
    );
}
