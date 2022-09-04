use rocket::serde::json::{Json, serde_json::json};

use crate::models::response::Response;

#[get("/", format = "any")]
fn index() -> Json<Response> {
    Json(Response {
        success: true,
        message: "Welcome to Lupus-Imap-Bridge".to_string(),
        data: json!(())
    })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}
