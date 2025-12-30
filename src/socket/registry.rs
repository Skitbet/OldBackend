use actix_ws::Session;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type WsRegistry = Arc<Mutex<HashMap<String, Session>>>;
