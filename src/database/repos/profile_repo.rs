use bson::{doc, to_document};
use futures::StreamExt;
use futures_util::TryStreamExt;
use mongodb::{Collection, Database};
use uuid::Uuid;

use crate::{models::profile::DBProfile, utils::error::AppError};
use crate::models::user::User;

#[derive(Clone)]
pub struct ProfileRepository {
    pub coll: Collection<DBProfile>,
}

impl ProfileRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            coll: db.collection("profiles"),
        }
    }

    pub async fn get_all_profiles(&self) -> Result<Vec<DBProfile>, AppError> {
        let mut cursor = self.coll.find(None, None)
            .await
            .map_err(|_| AppError::DBError)?;

        let mut profiles = Vec::new();

        while let Some(profile) = cursor.try_next().await.map_err(|_| AppError::DBError)? {
            profiles.push(profile);
        }

        Ok(profiles)
    }

    pub async fn create(&self, profile: &DBProfile) -> Result<(), AppError> {
        self.coll
            .insert_one(profile, None)
            .await
            .map_err(|_| AppError::DBError)?;
        Ok(())
    }

    pub async fn get_by_uuid(&self, uuid: &Uuid) -> Result<DBProfile, AppError> {
        let filter = doc! { "_id": uuid.to_string() };

        self.coll
            .find_one(filter, None)
            .await
            .map_err(|e| {
                log::error!("Profile DB failed: {e:?}");
                AppError::DBError
            })?
            .ok_or(AppError::ProfileNotFound)
    }

    pub async fn get_by_username(&self, username: &str) -> Result<DBProfile, AppError> {
        let filter = doc! { "username": username };

        self.coll
            .find_one(filter, None)
            .await
            .map_err(|e| {
                log::error!("Profile Fetch By Username DB failed: {e:?}");
                AppError::DBError
            })?
            .ok_or(AppError::ProfileNotFound)
    }

    pub async fn save(&self, uuid: &Uuid, profile: &DBProfile) -> Result<(), AppError> {
        let filter = doc! { "_id": uuid.to_string() };
        let update = doc! { "$set": to_document(profile)
            .map_err(|e| {
                log::error!("Profile Save DB failed: {e:?}");
                AppError::DBError
            })?
        };

        self.coll
            .update_one(filter, update, None)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_profiles_by_ids_and_usernames(
        &self,
        usernames: Vec<String>,
        ids: Vec<String>,
    ) -> Result<Vec<DBProfile>, AppError> {
        let mut or_filters = Vec::new();

        if !usernames.is_empty() {
            or_filters.push(doc! { "username": { "$in": usernames } });
        }

        if !ids.is_empty() {
            or_filters.push(doc! { "_id": { "$in": ids } });
        }

        if or_filters.is_empty() {
            return Ok(vec![]); // nothing to search for
        }

        let filter = doc! { "$or": or_filters };

        let mut cursor = self.coll.find(filter, None).await
            .map_err(|e| {
                log::error!("Getting profiles DB failed: {e:?}");
                AppError::DBError
            })?;

        let mut profiles = Vec::new();
        while let Some(profile) = cursor.next().await {
            profiles.push(profile.unwrap());
        }

        Ok(profiles)
    }
}
