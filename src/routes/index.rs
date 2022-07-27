use rocket::serde::json::Json;

use crate::models::response::Response;

#[get("/", format = "json")]
fn index() -> Json<Response> {
    Json(Response {
        success: true,
        message: "Welcome to Lupus-Imap-Bridge".to_string(),
    })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}
