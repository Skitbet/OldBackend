use chrono::{ DateTime, Utc };
use rand::distr::Alphanumeric;
use rand::Rng;
use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use crate::utils::uuid_as_string;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum CodeType {
    EmailVerify,
    PasswordReset,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub code: String,
    pub code_type: CodeType,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Code {
    pub fn new(email: String, username: String, code_type: CodeType) -> Self {
        let mut code_length = 12;
        if (code_type == CodeType::PasswordReset) {
            code_length = 5; 
        }
        
        let code = rand::rng().sample_iter(&Alphanumeric).take(code_length).map(char::from).collect();
        let created_at = Utc::now();
        let expires_at = created_at + chrono::Duration::minutes(10);
        Code {
            id: Uuid::new_v4(),
            email,
            username,
            code,
            code_type,
            expires_at,
            created_at,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
