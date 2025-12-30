use bson::{doc, to_document};
use mongodb::{Collection, Database};
use uuid::Uuid;

use crate::{models::settings::UserSettings, utils::error::AppError};

#[derive(Clone)]
pub struct SettingsRepository {
    coll: Collection<UserSettings>,
}

impl SettingsRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            coll: db.collection("users_settings"),
        }
    }

    pub async fn create(&self, settings: &UserSettings) -> Result<(), AppError> {
        self.coll
            .insert_one(settings, None)
            .await
            .map_err(|_| AppError::DBError)?;
        Ok(())
    }

    pub async fn get_by_uuid(&self, uuid: &Uuid) -> Result<UserSettings, AppError> {
        let filter = doc! { "_id": uuid.to_string() };

        self.coll
            .find_one(filter, None)
            .await
            .map_err(|_| AppError::DBError)?
            .ok_or(AppError::DBError)
    }

    pub async fn save(&self, uuid: &Uuid, settings: &UserSettings) -> Result<(), AppError> {
        let filter = doc! { "_id": uuid.to_string() };
        let update = doc! { "$set": to_document(settings).map_err(|e| AppError::InternalServerError(e.to_string()))? };

        self.coll
            .update_one(filter, update, None)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }
}
