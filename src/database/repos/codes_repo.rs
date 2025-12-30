use bson::doc;
use chrono::Utc;
use mongodb::{Collection, Database};
use uuid::Uuid;

use crate::models::codes::Code;
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct CodeRepository {
    coll: Collection<Code>,
}

impl CodeRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            coll: db.collection("codes"),
        }
    }

    pub async fn remove_expired(&self) -> mongodb::error::Result<u64> {
        let now = Utc::now();
        let result = self.coll
            .delete_many(doc! { "expires_at": { "$lte": now } }, None)
            .await?;

        Ok(result.deleted_count)
    }
    
    pub async fn create(&self, code: &Code) -> Result<(), AppError> {
        self.coll
            .insert_one(code, None)
            .await
            .map_err(|_: mongodb::error::Error| AppError::DBError)?;
        Ok(())
    }

    pub async fn exists(&self, code: &str) -> Result<bool, AppError> {
        let filter = doc! { "code": code };
        Ok(self
            .coll
            .find_one(filter, None)
            .await
            .map_err(|_: mongodb::error::Error| AppError::DBError)?
            .is_some())
    }
    
    pub async fn get_code_by_code(&self, code: &str) -> Result<Code, AppError> {
        let filter = doc! { "code": code };
        self
            .coll
            .find_one(filter, None)
            .await
            .map_err(|_| AppError::DBError)?
            .ok_or(AppError::InternalServerError("Code does not exists".into()))
    }
    
    pub async fn delete_code_by_code(&self, code: &str) -> Result<bool, AppError> {
        let filter = doc! { "code": code };
        let deleted_result = self.coll.delete_one(filter, None)
            .await
            .map_err(|_| AppError::DBError)?;
        
        Ok(deleted_result.deleted_count > 0)
    }

    pub async fn delete_code_by_uuid(&self, uuid: &Uuid) -> Result<bool, AppError> {
        let filter = doc! { "user_Id": uuid.to_string() };
        let deleted_result = self.coll.delete_one(filter, None)
            .await
            .map_err(|_| AppError::DBError)?;

        Ok(deleted_result.deleted_count > 0)
    }

}
