use redis::aio::ConnectionLike;
use redis::{pipe, AsyncCommands};
use redis::Pipeline;
use crate::models::post::Post;
use crate::models::profile::DBProfile;
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct PostCache {
    pub client: redis::Client,
}

impl PostCache {
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }
    
    pub async fn get(&self, id: &str) -> Result<Option<Post>, AppError> {
        let mut conn = self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                log::error!("Redis connection error in get: {:?}", e);
                AppError::InternalServerError("Redis connection failed".into())
            })?;

        let value: Option<String> = conn.get(format!("post:id:{id}")).await.ok();

        if let Some(json) = value {
            serde_json::from_str(&json).map(Some).map_err(|e| {
                log::error!("Failed to parse post from cache: {:?}", e);
                AppError::InternalServerError("Corrupt cache data".into())
            })
        } else {
            Ok(None)
        } 
    }

    pub async fn set(&self, post: &Post) -> Result<(), AppError> {
        let id = post.id.to_string();
        let mut conn = self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                log::error!("Redis connection error in set: {:?}", e);
                AppError::InternalServerError("Redis connection failed".into())
            })?;

        let json = serde_json::to_string(post)
            .map_err(|e| {
                log::error!("Failed to serialize post: {:?}", e);
                AppError::InternalServerError("Post serialization failed".into())
            })?;

        let _: () = conn.set_ex(format!("post:id:{id}"), json, 600)
            .await
            .map_err(|e| {
                log::error!("Redis set_ex failed: {:?}", e);
                AppError::InternalServerError("Failed to set post in cache".into())
            })?;

        Ok(())
    }

    pub async fn set_many(&self, posts: &[Post]) -> Result<(), AppError> {
        let mut conn = self.client.get_multiplexed_async_connection().await
            .map_err(|e| {
                log::error!("Redis connection failed: {:?}", e);
                AppError::InternalServerError("Redis connection failed".into())
            })?;

        let mut pipeline = pipe();

        for post in posts {
            if let Ok(json) = serde_json::to_string(post) {
                let key = format!("post:id:{}", post.id);
                pipeline.set_ex(key, json, 600);
            } else {
                log::warn!("Failed to serialize post for caching: {:?}", post.id);
            }
        }

        let _: Vec<()> = pipeline.query_async(&mut conn).await.map_err(|e| {
            log::error!("Redis pipeline execution failed: {:?}", e);
            AppError::InternalServerError("Redis pipeline failed".into())
        })?;

        Ok(())
    }
    
    pub async fn invalidate(&self, id: &str) -> Result<(), AppError> {
        let mut conn = self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                log::error!("Redis connection error in invalidate: {:?}", e);
                AppError::InternalServerError("Redis connection failed".into())
            })?;

        let _: () = conn.del(format!("post:id:{id}"))
            .await
            .map_err(|e| {
                log::error!("Failed to delete post from cache: {:?}", e);
                AppError::InternalServerError("Failed to delete post cache".into())
            })?;

        Ok(())
    }
}
