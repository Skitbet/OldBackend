use actix_web::{get, patch, web, HttpResponse, Responder};
use crate::middleware::admin_guard::AdminGuard;
use crate::models::post::{AdminPatchPost, PostResponse};
use crate::state::AppState;
use crate::utils::error::AppError;

pub fn config(web: &mut web::ServiceConfig) {
    web.service(
        web::scope("/posts")
            .service(get_posts)
    );
}

// Get all posts
#[get("/")]
pub async fn get_posts(
    _guard: AdminGuard,
    state: web::Data<AppState>
) -> Result<impl Responder, AppError> {
    let posts = state.services.post_service.get_all().await?;
    Ok(HttpResponse::Ok().json(posts))
}


#[patch("/update/{id}")]
async fn patch_post(
    path: web::Path<String>,
    patch: web::Json<AdminPatchPost>,
    _guard: AdminGuard,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();

    let updated = state.services.post_service.patch(&id, patch.into_inner()).await?;
    Ok(HttpResponse::Ok().json(PostResponse::from(&updated)))
}

