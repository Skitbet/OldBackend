use actix_web::{ HttpResponse, Responder, get, patch, post, web::{ self, Data } };
use bson::Document;
use log::info;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use crate::{ models::profile::PatchProfile, state::{ AppState }, utils::{ error::AppError } };
use crate::middleware::auther::Auther;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring /api/profile scope");
    cfg.service(
        web
            ::scope("profile")
            .service(get_public_user)
            .service(get_private_user)
            .service(patch_profile)
            .service(get_lookup_user)
            .service(follow_profile)
            .service(get_quicklookup)
    );
}

#[derive(Deserialize, ToSchema)]
pub struct QuickLookupData {
    username: Option<Vec<String>>,
    _id: Option<Vec<String>>,
}

#[derive(Serialize, ToSchema)]
pub struct QuickProfileLookup {
    _id: String,
    username: String,
    display_name: String,
    profile_picture: String,
}

#[derive(Serialize, ToSchema)]
pub struct QuickLookupResponse {
    cards: Vec<QuickProfileLookup>,
}

/**
 * Returns users public profile and increases the view count
 */
#[get("/{username}/public")]
pub async fn get_public_user(
    state: Data<AppState>,
    // req: HttpRequest,
    path: web::Path<String>
) -> Result<impl Responder, AppError> {
    let username = path.into_inner();

    // check profile exists for current user
    let mut profile = state.services.profile_service.get_by_username(&username).await?;
    profile.views += 1;

    state.services.profile_service.save(&profile.id, &profile).await?;
    Ok(HttpResponse::Ok().json(profile.to_public()))
}

/**
 * Returns users public profile
 */
#[get("/{username}/lookup")]
pub async fn get_lookup_user(
    state: Data<AppState>,
    // req: HttpRequest,
    path: web::Path<String>
) -> Result<impl Responder, AppError> {
    let username = path.into_inner();

    // check profile exists for current user
    let profile = state.services.profile_service.get_by_username(&username).await?;

    Ok(HttpResponse::Ok().json(profile.to_public()))
}

/**
 * Allows people to access their own data, data other people shouldn't be able to see.
 * For normal data /public is used.
 */
#[get("/me")]
pub async fn get_private_user(
    auther: Auther,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let session = auther.session;

    let profile = state.services.profile_service.get_by_uuid(&session.user_uuid).await?;
    Ok(HttpResponse::Ok().json(profile))
}

/**
 * Patches the PatchProfile data into the users profile in the db
 */
#[patch("/me")]
async fn patch_profile(
    auther: Auther,
    state: Data<AppState>,
    patch: web::Json<PatchProfile>
) -> Result<impl Responder, AppError> {
    let session = auther.session;
    let user_id = session.user_uuid;

    let mut patch_doc = Document::new();

    if let Some(display_name) = &patch.display_name {
        patch_doc.insert("display_name", display_name);
    }
    if let Some(bio) = &patch.bio {
        patch_doc.insert("bio", bio);
    }
    if let Some(pronouns) = &patch.pronouns {
        patch_doc.insert("pronouns", pronouns);
    }
    if let Some(languages) = &patch.languages {
        patch_doc.insert("languages", bson::to_bson(languages).unwrap());
    }
    if let Some(links) = &patch.links {
        patch_doc.insert("links", bson::to_bson(links).unwrap());
    }
    if let Some(status) = &patch.status {
        patch_doc.insert("status", status);
    }

    if patch_doc.is_empty() {
        return Err(AppError::BadRequest("No fields to update".into()));
    }

    let updated = state.services.profile_service.patch(&user_id, patch_doc).await?;
    Ok(HttpResponse::Ok().json(updated))
}

/**
 * Toggle Follow/UnFollow on a profile
 */
#[post("{username}/follow")]
pub async fn follow_profile(
    auther: Auther,
    state: Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder, AppError> {
    let username_to_follow = path.into_inner();
    let session = auther.session;
    let user_id = session.user_uuid;

    let mut user_profile = state.services.profile_service.get_by_uuid(&user_id).await?;
    let mut to_follow_profile = state.services.profile_service.get_by_username(
        &username_to_follow
    ).await?;

    if user_profile.username == to_follow_profile.username {
        return Err(AppError::InternalServerError("Cannot follow yourself!".into()));
    }

    // Check if user is already following
    let is_following = to_follow_profile.followers.contains(&user_id.to_string());

    if is_following {
        // Unfollow: remove from both sets
        to_follow_profile.followers.remove(&user_id.to_string());
        user_profile.following.remove(&to_follow_profile.id.to_string());
        state.services.profile_service.save(&to_follow_profile.id, &to_follow_profile).await?;
        state.services.profile_service.save(&user_profile.id, &user_profile).await?;
        Ok(HttpResponse::Ok().json(serde_json::json!({ "followed": false })))
    } else {
        // Follow: add to both sets
        to_follow_profile.followers.insert(user_id.to_string());
        user_profile.following.insert(to_follow_profile.id.to_string());
        state.services.profile_service.save(&to_follow_profile.id, &to_follow_profile).await?;
        state.services.profile_service.save(&user_profile.id, &user_profile).await?;
        Ok(HttpResponse::Ok().json(serde_json::json!({ "followed": true })))
    }
}

/**
 *   Quicklook for quickily getting data without returning whole profiles
 */
#[post("/quicklookup")]
pub async fn get_quicklookup(
    state: Data<AppState>,
    data: web::Json<QuickLookupData>
) -> Result<impl Responder, AppError> {
    let usernames = data.username.clone().unwrap_or_default();
    let ids = data._id.clone().unwrap_or_default();

    let result = state.services.profile_service.get_many(usernames, ids).await?;

    let mut cards = Vec::new();
    for profile in result {
        let profile = &profile;
        cards.push(QuickProfileLookup {
            _id: profile.id.clone().to_string(),
            username: profile.username.clone(),
            display_name: profile.display_name.clone(),
            profile_picture: profile.profile_picture.clone().unwrap(),
        });
    }

    Ok(HttpResponse::Ok().json(cards))
}
