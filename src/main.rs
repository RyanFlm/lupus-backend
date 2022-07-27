mod db;
mod models;
mod imap;

#[macro_use]
extern crate rocket;

use models::user::User;
use rocket::{serde::{json::Json, uuid::Uuid}};

#[get("/<id>")]
fn index(id: Uuid) -> Json<User> {
    Json(User::load(id))
}

#[get("/imap/folders")]
fn imap_get_folders() {
    // Load folders from IMAP server
    imap::test();
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, imap_get_folders])
}
