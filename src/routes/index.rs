use rocket::serde::json::{serde_json::json, Json};

use crate::models::response::Response;

#[get("/", format = "any")]
fn index() -> Json<Response> {
    Json(Response {
        success: true,
        message: "Welcome to Lupus-Imap-Bridge".to_string(),
        data: json!(()),
    })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}
