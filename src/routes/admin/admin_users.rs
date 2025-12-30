use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use crate::middleware::admin_guard::AdminGuard;
use crate::state::AppState;
use crate::utils::error::AppError;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring /api/admin/users scope");
    cfg.service(
        web::scope("users")
            .service(get_all_users)
    );
}

#[get("/")]
async fn get_all_users(_admin: AdminGuard, data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let users = data.db.users.get_all_users().await?;
    Ok(HttpResponse::Ok().json(users))
}