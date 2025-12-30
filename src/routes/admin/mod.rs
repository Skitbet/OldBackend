use actix_web::{ get, web, HttpResponse };
use log::info;
use crate::middleware::admin_guard::AdminGuard;

mod admin_users;
mod admin_profiles;
mod admin_posts;
mod admin_reporting;
mod admin_announcements;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring admin routes under /api/admin");

    cfg.service(
        web
            ::scope("/admin")
            .service(get_is_admin)
            .configure(admin_users::config)
            .configure(admin_profiles::config)
            .configure(admin_posts::config)
            .configure(admin_reporting::config)
            .configure(admin_announcements::config),
    );
}

/**
we'll let admin guard handle the authentication
*/
#[get("is-admin")]
pub async fn get_is_admin(_auther: AdminGuard) -> impl actix_web::Responder {
    HttpResponse::Ok().json(
        serde_json::json!({
            "admin": true,
            "message": "You are an admin"})
    )
}
