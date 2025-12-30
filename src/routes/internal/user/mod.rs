use actix_web::web;
use log::info;
use crate::routes::internal::user::handler::{
    get_user_posts,
    get_user_settings,
    patch_user_settings,
    request_password_reset,
    reset_password,
    verify_user_email,
};

mod handler;
pub mod types;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring /api/user scope");
    cfg.service(
        web
            ::scope("/user")
            .service(get_user_posts)
            .service(get_user_settings)
            .service(patch_user_settings)
            .service(verify_user_email)
            .service(request_password_reset)
            .service(reset_password)
    );
}
