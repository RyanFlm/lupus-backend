#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, uuid::Uuid};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    pub id: Uuid,
    pub name: String,
    pub age: u8,
}

#[get("/<id>")]
fn index(id: Uuid) -> Json<User> {
    Json(User {
        id,
        name: String::from("John"),
        age: 30,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
