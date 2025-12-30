use actix_cors::Cors;
use regex::Regex;

pub mod auth;
pub mod error;
pub mod json;
pub mod post;
pub mod r2endpoint;
pub mod hash;
pub mod roles;

/// (De)serialize `Uuid` as a string in JSON.
pub mod uuid_as_string {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use uuid::Uuid;

    pub fn serialize<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&uuid.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Uuid::parse_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Returns a default permissive CORS configuration.
/// Allows any origin, method, and header. 
pub fn default_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
        .send_wildcard()
}

/// Checks if a string is in a basic valid email format.
pub fn is_email(input: &str) -> bool {
    let email_regex = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap();
    email_regex.is_match(input)
}
