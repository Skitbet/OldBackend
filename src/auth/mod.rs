use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod password;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // subject (user ID)
    pub exp: usize,         // expiration time (as UTC timestamp)
    pub session_id: String, // session ID for tracking
}

pub fn create_jwt(
    user_uuid: &Uuid,
    session_id: &Uuid,
    secret: &str,
    expiration_sections: i64,
) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(expiration_sections))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_uuid.to_string(),
        exp: expiration as usize,
        session_id: session_id.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("JWT token creation should not fail")
}

// pub fn verify_jwt(token: &str, secret: &str) -> Option<Claims> {
//     let validation = Validation::default();
//     decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)
//     .map(|data| data.claims)
//     .ok()
// }
