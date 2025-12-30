use actix_web::{ post, web::{ self, Data }, HttpResponse, Responder };
use chrono::Utc;
use bson::doc;

use crate::{
    auth::password::{ hash_password, verify_password },
    middleware::auther::Auther,
    models::user,
    routes::internal::settings::types::PasswordChangeRequest,
    state::AppState,
    utils::error::AppError,
};

#[post("/change_password")]
pub async fn change_password(
    auther: Auther,
    state: Data<AppState>,
    body: web::Json<PasswordChangeRequest>
) -> Result<impl Responder, AppError> {
    let old_password = &body.current_password;
    let new_password = &body.new_password;

    let session = auther.session;
    let user_id = session.user_uuid;
    let user = state.db.users.get_by_uuid(&user_id).await?;
    if !verify_password(old_password, &user.password_hash) {
        return Err(AppError::Unauthorized("Current password field is incorrect".into()));
    }

    let hashed_pass = hash_password(&new_password).map_err(|e|
        AppError::InternalServerError(e.to_string())
    )?;

    let mut fields = doc! {};
    fields.insert("password_hash", hashed_pass);
    fields.insert("updated_at", Utc::now());

    state.db.users.update_fields(&user_id, fields).await?;
    Ok(HttpResponse::Ok().json("Password changed successfully"))
}
