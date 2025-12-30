use actix_web::{ get, patch, web, HttpResponse, Responder };
use log::info;
use uuid::Uuid;
use crate::middleware::admin_guard::AdminGuard;
use crate::models::profile::PatchDBProfile;
use crate::state::AppState;
use crate::utils::error::AppError;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring /api/admin/profiles scope");
    cfg.service(
        web::scope("profiles").service(get_all_profiles).service(get_profile).service(patch_profile)
    );
}

#[get("")]
async fn get_all_profiles(
    _admin: AdminGuard,
    data: web::Data<AppState>
) -> Result<impl Responder, AppError> {
    let profiles = data.db.profiles.get_all_profiles().await?;
    Ok(HttpResponse::Ok().json(profiles))
}

#[get("/fetch/{profile_id}")]
async fn get_profile(
    _admin: AdminGuard,
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder, AppError> {
    let id = Uuid::parse_str(&path.into_inner()).map_err(|_|
        AppError::BadRequest("Invalid ID".into())
    )?;
    let profile = data.services.profile_service.get_by_uuid(&id).await?;
    Ok(HttpResponse::Ok().json(profile))
}

#[patch("/update/{profile_id}")]
async fn patch_profile(
    _admin: AdminGuard,
    state: web::Data<AppState>,
    patch: web::Json<PatchDBProfile>,
    path: web::Path<String>
) -> Result<impl Responder, AppError> {
    let id: Uuid = Uuid::parse_str(&path.into_inner()).map_err(|_|
        AppError::BadRequest("Invalid ID".into())
    )?;

    let patch_profile = patch.into_inner();

    let updated = state.services.profile_service.patch(&id, patch_profile.to_mongo_doc()).await?;
    Ok(HttpResponse::Ok().json(updated))
}
