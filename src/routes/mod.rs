pub mod api_test;
pub mod internal;
pub mod admin;

use actix_web::web;
use log::info;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring global /api scope");

    cfg.service(
        web::scope("/api")
            .configure(internal::config)
            .configure(admin::config)
    );

    // Optional: still include this if it's a separate root route
    cfg.service(api_test::api_test);
}
