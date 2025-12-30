use crate::redis::cache::post_cache::PostCache;
use crate::redis::cache::profile_cache::ProfileCache;
use crate::utils::error::AppError;

pub mod cache;

#[derive(Clone)]
pub struct InkvaultCache {
    pub profile_cache: ProfileCache,
    pub post_cache: PostCache,
}

impl InkvaultCache {
    pub async fn new(redis_url: &str) -> Result<Self, AppError> {
        log::info!("Attempted to connect to Redis");
        let redis_client = redis::Client::open(redis_url).map_err(|_| AppError::InternalServerError("Failed to setup Redis".into()))?;
        log::info!("Successfully connected to Redis");
        
        Ok(Self {
            profile_cache: ProfileCache::new(redis_client.clone()),
            post_cache: PostCache::new(redis_client.clone()),
        })
    }
}
