use bson::doc;
use chrono::{Duration, Utc};
use mongodb::results::DeleteResult;
use mongodb::{Collection, Database};

use crate::models::user::PreRegisteredUser;
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct PreRegisterUserRepository {
    coll: Collection<PreRegisteredUser>,
}

impl PreRegisterUserRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            coll: db.collection("preusers"),
        }
    }
    
    pub async fn remove_expired(&self) -> mongodb::error::Result<u64> {
        let cutoff_time = Utc::now() - Duration::hours(24);
        
        let filter = doc! {
            "created_at": { "$lt": cutoff_time }
        };
        
        let result = self.coll.delete_many(filter, None).await?;
        Ok(result.deleted_count)
    }

    pub async fn create(&self, user: &PreRegisteredUser) -> Result<(), AppError> {
        self.coll
            .insert_one(user, None)
            .await
            .map_err(|_: mongodb::error::Error| AppError::DBError)?;
        Ok(())
    }

    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let filter = doc! { "email": email };
        Ok(self
            .coll
            .find_one(filter, None)
            .await
            .map_err(|_: mongodb::error::Error| AppError::DBError)?
            .is_some())
    }
    
    pub async fn delete(&self, pre_registered_user: &PreRegisteredUser) -> Result<DeleteResult, AppError> {
        let filter = doc! { "username": pre_registered_user.username.clone() };
        
        Ok(self.coll.delete_one(filter, None)
            .await
            .map_err(|_| AppError::DBError)?)
    }

    pub async fn get_by_username(&self, username: &str) -> Result<PreRegisteredUser, AppError> {
        let filter = doc! { "username": username };
        self.coll
            .find_one(filter, None)
            .await
            .map_err(|_: mongodb::error::Error| AppError::DBError)?
            .ok_or(AppError::UserNotFound)
    }

    pub async fn get_by_email(&self, email: &str) -> Result<PreRegisteredUser, AppError> {
        let filter = doc! { "email": email };
        self.coll
            .find_one(filter, None)
            .await
            .map_err(|_: mongodb::error::Error| AppError::DBError)?
            .ok_or(AppError::UserNotFound)
    }
}
