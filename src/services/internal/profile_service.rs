use std::collections::HashSet;
use bson::{doc, Document};
use uuid::Uuid;
use log::info;

use crate::database::repos::profile_repo::ProfileRepository;
use crate::models::profile::{DBProfile};
use crate::redis::cache::profile_cache::ProfileCache;
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct ProfileService {
    repo: ProfileRepository,
    cache: ProfileCache,
}

impl ProfileService {
    pub fn new(repo: ProfileRepository, cache: ProfileCache) -> Self {
        Self { repo, cache }
    }

    pub async fn get_by_uuid(&self, uuid: &Uuid) -> Result<DBProfile, AppError> {
        let uuid_str = uuid.to_string();

        if let Some(profile) = self.cache.get_by_uuid(&uuid_str).await? {
            info!("Cache hit for profile UUID: {uuid_str}");
            return Ok(profile);
        }

        info!("Cache miss for profile UUID: {uuid_str} — fetching from MongoDB");
        let profile = self.repo.get_by_uuid(uuid).await?;
        self.cache.set_by_uuid(&uuid_str, &profile).await?;
        Ok(profile)
    }

    pub async fn get_by_username(&self, username: &str) -> Result<DBProfile, AppError> {
        if let Some(profile) = self.cache.get_by_username(username).await? {
            info!("Cache hit for profile username: {username}");
            return Ok(profile);
        }

        info!("Cache miss for profile username: {username} — fetching from MongoDB");
        let profile = self.repo.get_by_username(username).await?;
        self.cache.set_by_username(username, &profile).await?;
        Ok(profile)
    }

    pub async fn get_many(&self, usernames: Vec<String>, ids: Vec<String>) -> Result<Vec<DBProfile>, AppError> {
        let mut found_profiles = Vec::new();
        let mut missing_usernames = Vec::new();
        let mut missing_ids = Vec::new();

        for id in &ids {
            match self.cache.get_by_uuid(id).await {
                Ok(Some(profile)) => {
                    info!("Cache hit for profile UUID: {id}");
                    found_profiles.push(profile);
                }
                _ => {
                    info!("Cache miss for profile UUID: {id}");
                    missing_ids.push(id.clone());
                }
            }
        }

        for username in &usernames {
            match self.cache.get_by_username(username).await {
                Ok(Some(profile)) => {
                    info!("Cache hit for profile username: {username}");
                    found_profiles.push(profile);
                }
                _ => {
                    info!("Cache miss for profile username: {username}");
                    missing_usernames.push(username.clone());
                }
            }
        }

        if !missing_usernames.is_empty() || !missing_ids.is_empty() {
            let fresh = self.repo
                .get_profiles_by_ids_and_usernames(missing_usernames, missing_ids)
                .await?;
            info!("Fetched {} profiles from MongoDB", fresh.len());

            for profile in &fresh {
                let uuid_str = profile.id.to_string();
                let _ = self.cache.set_by_uuid(&uuid_str, profile).await;
                let _ = self.cache.set_by_username(&profile.username, profile).await;
            }

            found_profiles.extend(fresh);
        }

        let mut seen_ids = HashSet::new();
        found_profiles.retain(|p| seen_ids.insert(p.id.clone()));
        Ok(found_profiles)
    }

    pub async fn save(&self, uuid: &Uuid, profile: &DBProfile) -> Result<(), AppError> {
        self.repo.save(uuid, profile).await?;
        self.cache.set_by_uuid(&uuid.to_string(), profile).await?;
        self.cache.set_by_username(&profile.username, profile).await?;
        info!("Saved profile to DB and updated cache for UUID: {} and username: {}", uuid, profile.username);
        Ok(())
    }

    pub async fn patch(&self, uuid: &Uuid, patch: Document) -> Result<DBProfile, AppError> {
        self
            .repo
            .coll
            .update_one(
                doc! { "_id": uuid.to_string() },
                doc! { "$set": patch },
                None,
            )
            .await
            .map_err(|e| {
                log::error!("Mongo error: {:?}", e);
                AppError::DBError
            })?;
        
        
        let updated = self.repo.get_by_uuid(&uuid).await?;
        
        let username = updated.username.clone();
        self.cache.invalidate(&uuid.to_string(), Some(&username)).await?;
        
        self.cache.set_by_uuid(&uuid.to_string(), &updated).await?;
        self.cache.set_by_username(&updated.username, &updated).await?;
        
        Ok(updated)
    }
}
