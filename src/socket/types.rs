use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
pub enum WsMessage {
    CONNECTION {
        #[serde(rename = "subtype")]
        subtype: String,
        data: Value,
        #[serde(default)]
        success: Option<bool>,
    },
    ANNOUNCEMENT {
        #[serde(rename = "subtype")]
        subtype: String,
        data: Value,
        #[serde(default)]
        success: Option<bool>,
    },
}