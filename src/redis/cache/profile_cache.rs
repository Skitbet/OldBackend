use redis::AsyncCommands;
use crate::models::profile::DBProfile;
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct ProfileCache {
    pub client: redis::Client,
}

impl ProfileCache {
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }

    pub async fn get_by_uuid(&self, uuid: &str) -> Result<Option<DBProfile>, AppError> {
        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| {
            log::error!("Redis connection failed for get_by_uuid: {e:?}");
            AppError::BadRequest("Redis fail".into())
        })?;

        let value: Option<String> = conn.get(format!("profile:id:{uuid}")).await.ok();

        if let Some(json) = value {
            serde_json::from_str(&json).map(Some).map_err(|e| {
                log::error!("Failed to parse cached profile for UUID {uuid}: {e:?}");
                AppError::InternalServerError("Invalid cache".into())
            })
        } else {
            Ok(None)
        }
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<DBProfile>, AppError> {
        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| {
            log::error!("Redis connection failed for get_by_username: {e:?}");
            AppError::BadRequest("Redis fail".into())
        })?;

        let value: Option<String> = conn.get(format!("profile:username:{username}")).await.ok();

        if let Some(json) = value {
            serde_json::from_str(&json).map(Some).map_err(|e| {
                log::error!("Failed to parse cached profile for username {username}: {e:?}");
                AppError::InternalServerError("Invalid cache".into())
            })
        } else {
            Ok(None)
        }
    }

    pub async fn set_by_uuid(&self, uuid: &str, profile: &DBProfile) -> Result<(), AppError> {
        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| {
            log::error!("Redis connection failed for set_by_uuid: {e:?}");
            AppError::BadRequest("Redis fail".into())
        })?;

        let json = serde_json::to_string(profile).map_err(|e| {
            log::error!("Serialization failed for set_by_uuid {uuid}: {e:?}");
            AppError::InternalServerError("Failed to create profile cache json".into())
        })?;

        let _: () = conn
            .set_ex(format!("profile:id:{uuid}"), json, 600)
            .await
            .map_err(|e| {
                log::error!("Redis set_ex failed for UUID {uuid}: {e:?}");
                AppError::InternalServerError("Failed to set profile cache json".into())
            })?;

        log::debug!("Added profile:id:{uuid}");
        Ok(())
    }

    pub async fn set_by_username(&self, username: &str, profile: &DBProfile) -> Result<(), AppError> {
        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| {
            log::error!("Redis connection failed for set_by_username: {e:?}");
            AppError::BadRequest("Redis fail".into())
        })?;


        let json = serde_json::to_string(profile).map_err(|e| {
            log::error!("Serialization failed for set_by_username {username}: {e:?}");
            AppError::InternalServerError("Failed to serialize profile".into())
        })?;

        let _: () = conn
            .set_ex(format!("profile:username:{username}"), json, 600)
            .await
            .map_err(|e| {
                log::error!("Redis set_ex failed for username {username}: {e:?}");
                AppError::InternalServerError("Failed to set profile cache".into())
            })?;

        log::debug!("Added profile:username:{username}");
        
        Ok(())
    }

    pub async fn invalidate(&self, uuid: &str, name: Option<&str>) -> Result<(), AppError> {
        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| {
            log::error!("Redis connection failed for invalidate {uuid}: {e:?}");
            AppError::InternalServerError("Failed to invalidate a profile in cache".into())
        })?;

        let _: () = conn
            .del(format!("profile:id:{uuid}"))
            .await
            .map_err(|e| {
                log::error!("Redis del failed for invalidate {uuid}: {e:?}");
                AppError::InternalServerError("Failed to delete a profile cache".into())
            })?;
        log::debug!("Removed profile:id:{uuid}");
        
        match name {
            Some(username) => {
                let _: () = conn
                    .del(format!("profile:username:{username}"))
                    .await
                    .map_err(|e| {
                        log::error!("Redis del failed for invalidate {uuid}: {e:?}");
                        AppError::InternalServerError("Failed to delete a profile cache".into())
                    })?;
                log::debug!("Removed profile:username:{username}");
            },
            None => ()
        }
        Ok(())
    }
}
