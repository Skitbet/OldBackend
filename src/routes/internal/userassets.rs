use actix_multipart::Multipart;
use actix_web::{ HttpResponse, Responder, post, web::{ self, BytesMut } };
use futures::StreamExt;
use log::info;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

use crate::{ state::AppState, utils::{ error::AppError } };
use crate::middleware::auther::Auther;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring /api/media scope");
    cfg.service(web::scope("media").service(upload_profile_picture).service(upload_banner));
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateUserAssetResponse {
    pub link: String,
}

#[post("/me/assets/profile_picture")]
pub async fn upload_profile_picture(
    auther: Auther,
    payload: Multipart,
    state: web::Data<AppState>
) -> Result<impl Responder, AppError> {
    let session = auther.session;

    // bcz we get the session uuid from the token, we only modifying my
    // the related user profile picture so no hax unless token stolen?
    let user_id = session.user_uuid;
    let user = state.db.users.get_by_uuid(&user_id).await?;

    let file_data = get_payload_byes(payload).await?;

    // if no data was sent
    if file_data.is_empty() {
        return Err(AppError::BadRequest("No file data provided".into()));
    }

    // get content type from file data
    let uploaded_url = state.r2
        .upload_user_asset(
            &user.username,
            crate::services::r2::UserAssetType::ProfilePicture,
            &*file_data.to_vec()
        ).await
        .map_err(|e| {
            eprintln!("Error uploading to R2: {:?}", e);
            AppError::InternalServerError("Failed to upload profile picture".into())
        })?;

    let mut profile = state.db.profiles.get_by_username(&user.username).await?;
    profile.profile_picture = Some(uploaded_url.clone());
    state.services.profile_service.save(&user_id, &profile).await?;

    Ok(
        HttpResponse::Ok().json(UpdateUserAssetResponse {
            link: uploaded_url,
        })
    )
}

#[post("/me/assets/banner")]
pub async fn upload_banner(
    auther: Auther,
    payload: Multipart,
    state: web::Data<AppState>
) -> Result<impl Responder, AppError> {
    let session = auther.session;

    // bcz we get the session uuid from the token, we only modify
    // the related user profile picture so no hax unless token stolen?
    let user_id = session.user_uuid;
    let user = state.db.users.get_by_uuid(&user_id).await?;

    let file_data = get_payload_byes(payload).await?;

    // if no data was sent
    if file_data.is_empty() {
        return Err(AppError::BadRequest("No file data provided".into()));
    }

    // get content type from file data
    let uploaded_url = state.r2
        .upload_user_asset(
            &user.username,
            crate::services::r2::UserAssetType::Banner,
            &*file_data.to_vec()
        ).await
        .map_err(|e| {
            eprintln!("Error uploading to R2: {:?}", e);
            AppError::InternalServerError("Failed to upload banner".into())
        })?;

    let mut profile = state.db.profiles.get_by_username(&user.username).await?;
    profile.banner_picture = Some(uploaded_url.clone());
    state.services.profile_service.save(&user_id, &profile).await?;

    Ok(
        HttpResponse::Ok().json(UpdateUserAssetResponse {
            link: uploaded_url.clone(),
        })
    )
}

async fn get_payload_byes(mut payload: Multipart) -> Result<BytesMut, AppError> {
    let mut file_data = BytesMut::new();
    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|e| {
            eprint!("Error processing multipart: {:?}", e);
            AppError::BadRequest("Invalid multipart data".into())
        })?;

        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|_| AppError::BadRequest("Failed to read chunk".into()))?;
            file_data.extend_from_slice(&data);
        }
    }
    Ok(file_data)
}
