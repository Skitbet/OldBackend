use bson::doc;
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{ Collection, Database };
use mongodb::options::FindOptions;
use crate::models::report::{ Report, ReportStatus };
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct ReportRepository {
    coll: Collection<Report>,
}

impl ReportRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            coll: db.collection("reports"),
        }
    }

    pub async fn create(&self, report: &Report) -> Result<(), AppError> {
        self.coll.insert_one(report, None).await.map_err(|_| AppError::DBError)?;
        Ok(())
    }

    pub async fn fetch_by_id(&self, report_id: &String) -> Result<Report, AppError> {
        let filter = doc! { "_id": report_id };
        let report = self.coll
            .find_one(filter, None).await
            .map_err(|_| AppError::DBError)?
            .ok_or(AppError::ReportNotFound)?;
        Ok(report)
    }

    pub async fn update_status(
        &self,
        report_id: String,
        new_status: ReportStatus
    ) -> Result<bool, AppError> {
        let filter = doc! { "_id": report_id };
        let update =
            doc! {
            "$set": {
                "status": new_status.to_string(),
                "updated_at": Utc::now()
            }
        };

        let result = self.coll
            .update_one(filter, update, None).await
            .map_err(|_| AppError::DBError)?;
        Ok(result.modified_count == 1)
    }

    pub async fn query_reports(
        &self,
        status: Option<ReportStatus>,
        limit: u64,
        offset: u64,
        sort_by: &str,
        sort_order: &str
    ) -> Result<Vec<Report>, AppError> {
        let mut filter = doc! {};
        if let Some(status) = status {
            filter.insert("status", status.to_string());
        }

        let sort_direction = match sort_order.to_lowercase().as_str() {
            "asc" => 1,
            "desc" => -1,
            _ => -1,
        };

        let sort_doc = doc! { sort_by: sort_direction };

        let find_options = FindOptions::builder()
            .limit(limit as i64)
            .skip(offset)
            .sort(sort_doc)
            .build();

        let cursor = self.coll.find(filter, find_options).await.map_err(|e| {
            log::error!("Failed to query reports {}", e);
            return AppError::DBError;
        })?;

        let reports: Vec<Report> = cursor.try_collect().await.map_err(|e| {
            log::error!("Failed to query reports {}", e);
            return AppError::DBError;
        })?;

        Ok(reports)
    }

    // pub async fn delete_by_id(&self, id: &str) -> Result<bool, AppError> {
    //     let filter = doc! { "_id": id };
    //     let result = self
    //         .coll
    //         .delete_one(filter, None)
    //         .await
    //         .map_err(|_| AppError::DBError)?;
    //
    //     Ok(result.deleted_count > 0)
    // }
}
