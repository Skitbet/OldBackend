use bson::{doc, to_document};
use futures_util::TryStreamExt;
use mongodb::{Collection, Database};
use uuid::Uuid;

use crate::{models::user::User, utils::error::AppError};

#[derive(Clone)]
pub struct UserRepository {
    pub coll: Collection<User>,
}

impl UserRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            coll: db.collection("users"),
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        let mut cursor = self.coll.find(None, None)
            .await
            .map_err(|_| AppError::DBError)?;

        let mut users = Vec::new();

        while let Some(user) = cursor.try_next().await.map_err(|_| AppError::DBError)? {
            users.push(user);
        }

        Ok(users)
    }
    
    pub async fn create(&self, user: &User) -> Result<(), AppError> {
        self.coll
            .insert_one(user, None)
            .await
            .map_err(|_: mongodb::error::Error| AppError::DBError)?;
        Ok(())
    }

    pub async fn exists(&self, username: &str) -> Result<bool, AppError> {
        let filter = doc! { "username": username };
        Ok(self
            .coll
            .find_one(filter, None)
            .await
            .map_err(|_: mongodb::error::Error| AppError::DBError)?
            .is_some())
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

    pub async fn get_by_uuid(&self, user_uuid: &Uuid) -> Result<User, AppError> {
        let filter = doc! { "_id": user_uuid.to_string() };
        self.coll
            .find_one(filter, None)
            .await
            .map_err(|_: mongodb::error::Error| AppError::DBError)?
            .ok_or(AppError::UserNotFound)
    }

    pub async fn get_by_username(&self, username: &str) -> Result<User, AppError> {
        let filter = doc! { "username": username };
        self.coll
            .find_one(filter, None)
            .await
            .map_err(|_: mongodb::error::Error| AppError::DBError)?
            .ok_or(AppError::UserNotFound)
    }

    pub async fn get_by_email(&self, email: &str) -> Result<User, AppError> {
        let filter = doc! { "email": email };
        self.coll
            .find_one(filter, None)
            .await
            .map_err(|_: mongodb::error::Error| AppError::DBError)?
            .ok_or(AppError::UserNotFound)
    }

    pub async fn save(&self, user: &User) -> Result<(), AppError> {
        let filter = doc! { "_id": user.id.to_string() };
        let update = doc! { "$set": to_document(user).map_err(|e| AppError::InternalServerError(e.to_string()))? };

        self.coll
            .update_one(filter, update, None)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn update_fields(&self, user_uuid: &Uuid, fields: bson::Document) -> Result<(), AppError> {
        let filter = doc! { "_id": user_uuid.to_string() };
        let update = doc! { "$set": fields };

        self.coll
            .update_one(filter, update, None)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }
}
