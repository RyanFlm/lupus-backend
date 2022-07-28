mod models;
mod routes;
#[cfg(test)]
mod tests;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes::routes())
}
