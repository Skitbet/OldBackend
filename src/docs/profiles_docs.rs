#![allow(dead_code)]

use crate::{
    models::profile::{ DBProfile, PatchProfile },
    routes::internal::profile::{ QuickLookupData, QuickLookupResponse },
};

#[utoipa::path(
    get,
    path = "/api/profile/{username}/public",
    params(("username" = String, Path, description = "Username of the profile to fetch")),
    responses(
        (status = 200, description = "Public profile returned", body = DBProfile),
        (status = 404, description = "Profile not found")
    ),
    tag = "Profile"
)]
pub fn get_public_user() {}

#[utoipa::path(
    get,
    path = "/api/profile/{username}/lookup",
    params(("username" = String, Path, description = "Username of the profile to lookup")),
    responses(
        (status = 200, description = "Public lookup profile returned", body = DBProfile),
        (status = 404, description = "Profile not found")
    ),
    tag = "Profile"
)]
pub fn get_lookup_user() {}

#[utoipa::path(
    get,
    path = "/api/profile/me",
    params((
        "Authorization" = String,
        Header,
        description = "Bearer token for user authentication.",
    )),
    responses(
        (status = 200, description = "Private user profile returned", body = DBProfile),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Profile"
)]
pub fn get_private_user() {}

#[utoipa::path(
    patch,
    path = "/api/profile/me",
    params((
        "Authorization" = String,
        Header,
        description = "Bearer token for authenticated user.",
    )),
    request_body(content = PatchProfile, description = "Profile fields to update"),
    responses(
        (status = 200, description = "Profile updated successfully", body = DBProfile),
        (status = 400, description = "No valid fields to update"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Profile"
)]
pub fn patch_profile() {}

#[utoipa::path(
    post,
    path = "/api/profile/{username}/follow",
    params(
        ("Authorization" = String, Header, description = "Bearer token for authenticated user."),
        ("username" = String, Path, description = "Username of the profile to follow or unfollow")
    ),
    responses(
        (status = 200, description = "Follow state toggled successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Profile"
)]
pub fn follow_profile() {}

#[utoipa::path(
    post,
    path = "/api/profile/quicklookup",
    request_body(content = QuickLookupData, description = "Usernames or IDs to lookup"),
    responses(
        (status = 200, description = "Quick profile cards returned", body = QuickLookupResponse),
        (status = 404, description = "No profiles found")
    ),
    tag = "Profile"
)]
pub fn get_quicklookup() {}
