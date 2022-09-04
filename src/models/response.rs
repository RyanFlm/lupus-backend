use rocket::serde::json::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub message: String,
    pub data: Value,
}
