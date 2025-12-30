use bson::doc;
use chrono::Utc;
use mongodb::{Collection, Database};
use uuid::Uuid;

use crate::{models::session::Session, utils::error::AppError};

#[derive(Clone)]
pub struct SessionRepository {
    pub coll: Collection<Session>,
}

impl SessionRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            coll: db.collection("sessions"),
        }
    }

    pub async fn remove_expired(&self) -> mongodb::error::Result<u64> {
        let now = Utc::now();
        let result = self.coll
            .delete_many(doc! { "expires_at": { "$lte": now } }, None)
            .await?;

        Ok(result.deleted_count)
    }
    
    pub async fn create(&self, session: &Session) -> Result<(), AppError> {
        self.coll
            .insert_one(session, None)
            .await
            .map_err(|_| AppError::DBError)?;
        Ok(())
    }

    pub async fn get_by_token(&self, token: &str) -> Result<Session, AppError> {
        let filter = doc! { "token": token };

        self.coll
            .find_one(filter, None)
            .await
            .map_err(|_| AppError::DBError)?
            .ok_or(AppError::SessionNotFound)
    }

    pub async fn delete_by_token(&self, token: &str) -> Result<bool, AppError> {
        let filter = doc! { "token": token };

        let result = self
            .coll
            .delete_one(filter, None)
            .await
            .map_err(|_| AppError::DBError)?;

        Ok(result.deleted_count > 0)
    }

    pub async fn delete_all_for_user(&self, user_uuid: &Uuid) -> Result<u64, AppError> {
        let filter = doc! { "user_uuid": user_uuid.to_string() };

        let result = self
            .coll
            .delete_many(filter, None)
            .await
            .map_err(|_| AppError::DBError)?;

        Ok(result.deleted_count)
    }

    pub async fn refresh_expiration(&self, token: &str, new_expiry: i64) -> Result<(), AppError> {
        let filter = doc! { "token": token };
        let update = doc! { "$set": { "expires_at": new_expiry } };

        self.coll
            .update_one(filter, update, None)
            .await
            .map_err(|_| AppError::DBError)?;

        Ok(())
    }
}
