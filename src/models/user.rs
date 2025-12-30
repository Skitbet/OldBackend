use chrono::Utc;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use uuid::Uuid;
use crate::utils::roles::Role;
use crate::utils::uuid_as_string;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub id: Uuid,
    pub email: String,
    pub premium: bool,
    pub verified: bool,
    pub username: String,
    pub password_hash: String,
    // #[serde(default)]
    // pub role: Vec<Role>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: chrono::DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub last_login: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreRegisteredUser {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: chrono::DateTime<Utc>,
}

impl User {
    pub fn new(uuid: &Uuid, email: &String, username: &String, password_hash: &String) -> Self {
        let now = Utc::now();
        User {
            id: *uuid,
            email: email.to_string(),
            premium: false,
            verified: false,
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            // role: vec![Role::User],
            created_at: now,
            last_login: now,
        }
    }
}

impl PreRegisteredUser {
    pub fn new(email: &String, username: &String, password_hash: &String) -> Self {
        let now = Utc::now();
        PreRegisteredUser {
            id: Uuid::new_v4(),
            email: email.to_string(),
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            created_at: now,
        }
    }
}
