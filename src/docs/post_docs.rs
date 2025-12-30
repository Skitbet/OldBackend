#![allow(dead_code)]

use crate::routes::internal::posts::types::UserPatchPost;
use crate::models::post::PostResponse;

#[utoipa::path(
    get,
    path = "/api/posts",
    params(
        ("query" = Option<String>, Query, description = "Search query text"),
        ("tags" = Option<String>, Query, description = "Comma-separated list of tags"),
        ("author" = Option<String>, Query, description = "Filter by author username"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of posts successfully fetched", body = [PostResponse]),
        (status = 500, description = "Internal server error")
    ),
    tag = "Posts"
)]
pub async fn search_posts() {}

#[utoipa::path(
    post,
    path = "/api/posts/new",
    request_body(content = UserPatchPost, description = "Data for creating a post"),
    responses(
        (status = 200, description = "Post created successfully", body = PostResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    params((
        "Authorization" = String,
        Header,
        description = "Bearer token for user creating post.",
    )),
    tag = "Posts"
)]
pub async fn create_post() {}

#[utoipa::path(
    patch,
    path = "/api/posts/edit/{id}",
    params(("id" = String, Path, description = "Post UUID")),
    request_body(content = UserPatchPost, description = "Data for updating a post"),
    responses(
        (status = 200, description = "Post updated successfully", body = PostResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    params(("Authorization" = String, Header, description = "Bearer token for post owner.")),
    tag = "Posts"
)]
pub async fn edit_post() {}

#[utoipa::path(
    get,
    path = "/api/posts/id/{id}",
    params(("id" = String, Path, description = "Post UUID")),
    responses(
        (status = 200, description = "Post fetched successfully", body = PostResponse),
        (status = 404, description = "Post not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Posts"
)]
pub async fn get_post_by_id() {}

#[utoipa::path(
    delete,
    path = "/api/posts/delete/{id}",
    params(("id" = String, Path, description = "Post UUID")),
    responses(
        (status = 200, description = "Post deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    params(("Authorization" = String, Header, description = "Bearer token for post owner.")),
    tag = "Posts"
)]
pub async fn delete_post() {}

#[utoipa::path(
    get,
    path = "/api/posts/latest",
    params(
        ("amount" = Option<u64>, Query, description = "Number of posts to fetch"),
        ("displacement" = Option<u64>, Query, description = "Number of posts to skip")
    ),
    responses(
        (status = 200, description = "Latest posts fetched successfully", body = [PostResponse])
    ),
    tag = "Posts"
)]
pub async fn get_latest_posts() {}

#[utoipa::path(
    get,
    path = "/api/posts/popular",
    params(
        ("amount" = Option<u64>, Query, description = "Number of posts to fetch"),
        ("displacement" = Option<u64>, Query, description = "Number of posts to skip")
    ),
    responses(
        (status = 200, description = "Popular posts fetched successfully", body = [PostResponse])
    ),
    tag = "Posts"
)]
pub async fn get_popular_posts() {}

#[utoipa::path(
    get,
    path = "/api/posts/premium",
    params(
        ("amount" = Option<u64>, Query, description = "Number of posts to fetch"),
        ("displacement" = Option<u64>, Query, description = "Number of posts to skip")
    ),
    responses(
        (status = 200, description = "Premium posts fetched successfully", body = [PostResponse])
    ),
    tag = "Posts"
)]
pub async fn get_premium_posts() {}

#[utoipa::path(
    get,
    path = "/api/posts/random",
    params(
        ("amount" = Option<u64>, Query, description = "Number of posts to fetch")
    ),
    responses(
        (status = 200, description = "Random posts fetched successfully", body = [PostResponse])
    ),
    tag = "Posts"
)]
pub async fn get_random_posts() {}

#[utoipa::path(
    get,
    path = "/api/posts/by/{username}/{short_id}",
    params(
        ("username" = String, Path, description = "Author username"),
        ("short_id" = String, Path, description = "Short post ID")
    ),
    responses(
        (status = 200, description = "Post fetched successfully", body = PostResponse),
        (status = 404, description = "Post not found")
    ),
    tag = "Posts"
)]
pub async fn get_post() {}

#[utoipa::path(
    get,
    path = "/api/posts/random",
    responses((status = 200, body = PostResponse), (status = 404, description = "Post not found")),
    tag = "Posts"
)]
pub async fn get_a_random_post() {}

#[utoipa::path(
    post,
    path = "/api/posts/{id}/like",
    params(("id" = String, Path, description = "Post UUID")),
    responses((status = 200, description = "Post liked/unliked successfully")),
    tag = "Posts",
    params(("Authorization" = String, Header, description = "Bearer token for user."))
)]
pub async fn like_post() {}

#[utoipa::path(
    post,
    path = "/api/posts/{id}/dislike",
    params(("id" = String, Path, description = "Post UUID")),
    responses((status = 200, description = "Post disliked/undisliked successfully")),
    tag = "Posts",
    params(("Authorization" = String, Header, description = "Bearer token for user."))
)]
pub async fn dislike_post() {}
