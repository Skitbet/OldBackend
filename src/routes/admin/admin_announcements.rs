use actix_web::{get, post, web, Responder};
use serde::Deserialize;
use crate::middleware::admin_guard::AdminGuard;
use crate::state::AppState;
use crate::utils::error::AppError;

pub fn config(web: &mut web::ServiceConfig) {
    web.service(
        web::scope("/announcements")
            .service(new_announcement)
    );
}

#[derive(Debug, Deserialize)]
pub struct NewAnnouncementBody {
    pub title: String,
    pub body: String,
}

#[post("/new")]
async fn new_announcement(
    body: web::Json<NewAnnouncementBody>,
    _guard: AdminGuard,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    match state.services.announcement_service
        .new_announcement(&body.title, &body.body)
        .await
    {
        Ok(announcement) => Ok(actix_web::HttpResponse::Ok().json(announcement)),
        Err(e) => Err(AppError::InternalServerError("Error with announcements".to_string())),
    }
}
