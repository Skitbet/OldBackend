use actix_web::web;
use log::info;

pub mod profile;
pub mod session;
pub mod user;
pub mod userassets;
pub mod comment;
pub mod reporting;
pub mod posts;
pub mod settings;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring internal routes under /api");

    cfg.configure(session::config)
        .configure(profile::config)
        .configure(userassets::config)
        .configure(posts::config)
        .configure(user::config)
        .configure(comment::config)
        .configure(reporting::config)
        .configure(settings::config);
}
