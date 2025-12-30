use serde::Deserialize;
use utoipa::{ ToSchema, IntoParams };

// params for grabbing latest posts
#[derive(Debug, Deserialize)]
pub struct LatestPostParams {
    pub amount: Option<u64>, // how many posts to grab
    pub displacement: Option<u64>, // basically "skip this many posts"
}

// params if we wanna query posts with more options
#[derive(Debug, Deserialize, IntoParams)]
pub struct QueryPostsParams {
    pub limit: Option<u64>, // max posts per request
    pub page: Option<String>, // what page we're on (could’ve been u32 but meh)
    pub tags: Option<String>, // filter posts by tag(s)
}

// params for searching posts in a more detailed way
#[derive(Debug, Deserialize)]
pub struct PostSearchQuery {
    pub query: Option<String>, // text search (keywords n stuff)
    pub tags: Option<String>, // filter by tags again
    pub author: Option<String>, // filter by author name/id
    pub page: Option<u32>, // which page of results
    pub limit: Option<u32>, // how many per page
    pub sort: Option<String>, // sorting method (newest, oldest, etc.)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserPatchPost {
    pub title: Option<String>,
    pub body: Option<Option<String>>, // some(None) = clear body, None = leave unchanged
    pub tags: Option<Vec<String>>,
    pub nsfw: Option<bool>,
}
