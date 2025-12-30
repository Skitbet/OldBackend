use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{ models::user::User, utils::uuid_as_string };

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct UserSettings {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub id: Uuid,
    pub theme: Option<String>,
    pub page_length: u32,
    pub nsfw: bool,
    pub notifications: Option<NotificationSettings>,
    pub privacy: Option<PrivacySettings>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct NotificationSettings {
    pub email: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct PrivacySettings {}

impl UserSettings {
    pub fn new_from_user(user: &User) -> Self {
        let notifi_settings = NotificationSettings { email: Some(true) };

        let privacy_settings = PrivacySettings {};

        UserSettings {
            id: user.id,
            theme: Some("dark".into()),
            page_length: 20,
            nsfw: false,
            notifications: Some(notifi_settings),
            privacy: Some(privacy_settings),
        }
    }
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(), // will be overwritten
            theme: Some("dark".into()),
            page_length: 20,
            nsfw: false,
            notifications: Some(NotificationSettings { email: Some(true) }),
            privacy: Some(PrivacySettings {}),
        }
    }
}
