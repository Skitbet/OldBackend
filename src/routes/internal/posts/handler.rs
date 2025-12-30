use crate::middleware::auther::Auther;
use crate::models::media::{ Media, MediaMetadata };
use crate::models::post::{ Post, PostResponse };
use crate::routes::internal::posts::types::{ LatestPostParams, PostSearchQuery, UserPatchPost };
use crate::routes::internal::posts::upload::parse_multipart;
use crate::state::AppState;
use crate::utils::error::AppError;
use crate::utils::post::PostType;
use actix_multipart::Multipart;
use actix_web::web::{ Data, Json, Path, Query };
use actix_web::{ delete, get, patch, post, HttpResponse, Responder };
use bson::doc;
use chrono::Utc;
use futures_util::TryStreamExt;
use mongodb::options::FindOptions;
use serde_json::json;
use std::collections::HashSet;
use uuid::Uuid;

#[get("")]
pub async fn search_posts(
    query: Query<PostSearchQuery>,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20).min(100);
    let skip = (page - 1) * limit;

    let mut filter = doc! {};

    // try $text search if query provided
    let mut used_regex = false;
    if let Some(ref q) = query.query {
        filter.insert("$text", doc! { "$search": q });
    }

    // tags
    if let Some(ref tag_str) = query.tags {
        let tags: Vec<&str> = tag_str.split(',').collect();
        filter.insert("tags", doc! { "$in": tags });
    }

    // author
    if let Some(ref author_name) = query.author {
        let author = state.services.profile_service.get_by_username(author_name).await?;
        filter.insert("author_id", author.id.to_string());
    }

    // sorting
    let sort = match query.sort.as_deref() {
        Some("createdAt_asc") => doc! { "created_at": 1 },
        Some("createdAt_desc") => doc! { "created_at": -1 },
        Some("likes_desc") => doc! { "likes": -1 },
        _ => doc! { "created_at": -1 },
    };

    // run query
    let mut cursor = state.db.posts.coll
        .find(
            filter.clone(),
            FindOptions::builder()
                .skip(Some(skip.into()))
                .limit(Some(limit.into()))
                .sort(sort.clone())
                .build()
        ).await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let mut posts = Vec::new();
    while
        let Some(result) = cursor
            .try_next().await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
    {
        posts.push(result);
    }

    if posts.is_empty() {
        if let Some(ref q) = query.query {
            used_regex = true;
            let regex_filter =
                doc! {
                "$or": [
                    { "title": { "$regex": q, "$options": "i" } },
                    { "content": { "$regex": q, "$options": "i" } }
                ]
            };

            let mut cursor = state.db.posts.coll
                .find(
                    regex_filter,
                    FindOptions::builder()
                        .skip(Some(skip.into()))
                        .limit(Some(limit.into()))
                        .sort(sort)
                        .build()
                ).await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

            while
                let Some(result) = cursor
                    .try_next().await
                    .map_err(|e| AppError::InternalServerError(e.to_string()))?
            {
                posts.push(result);
            }
        }
    }

    // respond
    let response: Vec<PostResponse> = posts.iter().map(PostResponse::from).collect();
    Ok(
        HttpResponse::Ok().json(
            json!({
        "total": response.len(),
        "page": page,
        "limit": limit,
        "used_regex": used_regex, // debug flag
        "posts": response,
    })
        )
    )
}

#[post("/new")]
async fn create_post(
    auther: Auther,
    payload: Multipart,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let session = auther.session;
    let user = state.db.users.get_by_uuid(&session.user_uuid).await?;

    let (mut fields, media_files) = parse_multipart(payload).await?;

    // extract and validate required form fields
    let post_uuid = Uuid::new_v4();
    let title = fields.remove("title").ok_or(AppError::BadRequest("Missing title".into()))?;
    let body = fields.remove("body");
    let tags = fields
        .remove("tags")
        .map(|s| { serde_json::from_str::<Vec<String>>(&s).unwrap_or_default() })
        .unwrap_or_default();
    let mut nsfw = fields
        .get("nsfw")
        .map(|v| v == "true")
        .unwrap_or(false);

    if tags.contains(&String::from("nsfw")) {
        nsfw = true;
    }

    let post_type = PostType::Generic;

    // upload media to R2
    let mut media = Vec::new();
    for file in media_files {
        state.r2
            .upload_post_asset(post_uuid, post_type.clone(), &file.filename, &file.data).await
            .map_err(|_| AppError::InternalServerError("Failed to upload media".into()))?;

        let url = format!(
            "https://{}/postassets/{}/{}",
            state.cdn_domain,
            post_uuid,
            file.filename
        );
        media.push(Media {
            url,
            filename: file.filename,
            content_type: file.content_type,
            size_bytes: file.data.len() as u64,
            uploaded_at: Utc::now(),
            is_nsfw: None,

            metadata: MediaMetadata::Post {
                width: None,
                height: None,
                duration_secs: None,
            },
        });
    }

    let post = Post {
        id: post_uuid,
        author: user.username,
        author_id: session.user_uuid,
        title,
        body,
        tags,
        post_type,
        nsfw,
        likes: HashSet::new(),
        dislikes: HashSet::new(),
        media,
        created_at: Utc::now(),
    };

    state.services.post_service.create(&post).await?;
    Ok(HttpResponse::Ok().json(PostResponse::from(&post)))
}

#[patch("/edit/{id}")]
async fn edit_post(
    auther: Auther,
    path: Path<String>,
    payload: Json<UserPatchPost>,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let session = auther.session;
    let post_id = path.into_inner();

    // get the post model
    let mut post = state.services.post_service.get_by_id(&post_id).await?;

    // check if we are allowed to edit thi posts
    if post.author_id != session.user_uuid {
        return Err(AppError::Unauthorized("You are not the author of this post".into()))?;
    }

    if let Some(title) = &payload.title {
        post.title = title.clone();
    }
    if let Some(body) = &payload.body {
        post.body = body.clone();
    }
    if let Some(tags) = &payload.tags {
        post.tags = tags.clone();
    }
    if let Some(nsfw) = payload.nsfw {
        post.nsfw = nsfw;
    }

    // Save changes
    state.services.post_service.save(&post).await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(&post)))
}

#[get("/id/{id}")]
async fn get_post_by_id(
    path: Path<String>,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let post = state.services.post_service.get_by_id(&id).await?;
    Ok(HttpResponse::Ok().json(PostResponse::from(&post)))
}

#[delete("/delete/{id}")]
async fn delete_post(
    auther: Auther,
    path: Path<String>,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let session = auther.session;
    // let user = state.db.users.get_by_uuid(&session.user_uuid).await?;
    let post = state.services.post_service.get_by_id(&path.into_inner()).await?;
    if post.author_id != session.user_uuid {
        return Err(AppError::Unauthorized("You are not the author of this post".into()))?;
    }

    state.services.post_service.delete(&post.id.to_string()).await?;

    Ok(HttpResponse::Ok().json(json!({ "success": true })))
}

#[get("/latest")]
async fn get_latest_posts(
    state: Data<AppState>,
    query: Query<LatestPostParams>
) -> Result<impl Responder, AppError> {
    let amount = query.amount.unwrap_or(50);
    let displacement = query.displacement.unwrap_or(0);
    let posts = state.services.post_service.get_latest(amount, displacement).await?;
    let response: Vec<PostResponse> = posts.iter().map(PostResponse::from).collect();
    Ok(HttpResponse::Ok().json(response))
}

#[get("/popular")]
async fn get_popular_posts(
    state: Data<AppState>,
    query: Query<LatestPostParams>
) -> Result<impl Responder, AppError> {
    let amount = query.amount.unwrap_or(50);
    let displacement = query.displacement.unwrap_or(0);

    let posts = state.services.post_service.get_popular(amount, displacement, None).await?;

    let response: Vec<PostResponse> = posts.iter().map(PostResponse::from).collect();
    Ok(HttpResponse::Ok().json(response))
}

#[get("/premium")]
async fn get_premium_posts(
    state: Data<AppState>,
    query: Query<LatestPostParams>
) -> Result<impl Responder, AppError> {
    let amount = query.amount.unwrap_or(50);
    let displacement = query.displacement.unwrap_or(0);

    let posts = state.services.post_service.get_premium(amount, displacement, None).await?;

    let response: Vec<PostResponse> = posts.iter().map(PostResponse::from).collect();
    Ok(HttpResponse::Ok().json(response))
}

#[get("/random")]
async fn get_random_posts(
    state: Data<AppState>,
    query: Query<LatestPostParams>
) -> Result<impl Responder, AppError> {
    let amount = query.amount.unwrap_or(50);

    let posts = state.services.post_service.get_random(amount, None).await?;

    let response: Vec<PostResponse> = posts.iter().map(PostResponse::from).collect();
    Ok(HttpResponse::Ok().json(response))
}

#[get("/by/{username}/{short_id}")]
async fn get_post(
    path: Path<(String, String)>,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let (username, short_id) = path.into_inner();
    let post = state.services.post_service.find_by_author_and_short_id(&username, &short_id).await?;
    Ok(HttpResponse::Ok().json(PostResponse::from(&post)))
}

#[get("/random")]
async fn get_a_random_post(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let posts = state.services.post_service.get_random(1, None).await?;
    if let Some(post) = posts.first() {
        Ok(HttpResponse::Ok().json(PostResponse::from(post)))
    } else {
        Err(AppError::PostNotFound)
    }
}

#[post("/{id}/like")]
async fn like_post(
    auther: Auther,
    path: Path<String>,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let session = auther.session;
    let user_id = session.user_uuid.to_string();
    let mut post = state.services.post_service.get_by_id(&path.into_inner()).await?;

    // like toggle logic
    let liked = post.likes.insert(user_id.clone());
    if liked {
        post.dislikes.remove(&user_id); // remove old dislike
    } else {
        post.likes.remove(&user_id); // undo like
    }

    state.services.post_service.save(&post).await?;
    Ok(HttpResponse::Ok().json(json!({ "liked": liked })))
}

#[post("/{id}/dislike")]
async fn dislike_post(
    auther: Auther,
    path: Path<String>,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let session = auther.session;
    let user_id = session.user_uuid.to_string();
    let mut post = state.services.post_service.get_by_id(&path.into_inner()).await?;

    // dislike toggle logic
    let disliked = post.dislikes.insert(user_id.clone());
    if disliked {
        post.likes.remove(&user_id); // remove old like
    } else {
        post.dislikes.remove(&user_id); // undo dislike
    }

    state.services.post_service.save(&post).await?;
    Ok(HttpResponse::Ok().json(json!({ "disliked": disliked })))
}
