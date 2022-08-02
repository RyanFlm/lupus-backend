use crate::utils::imap::{connect, ConnectOptions};
use rocket::serde::json::Json;

#[get("/imap/folders?<host>&<username>&<password>")]
fn imap_get_folders(host: String, username: String, password: String) -> Json<Vec<String>> {
    let mut session = connect(ConnectOptions {
        host,
        port: 143,
        username,
        password,
        starttls: true,
    })
    .expect("Failed to connect to IMAP server");

    let folder_list = session.list(None, Some("%")).map_err(|e| e).unwrap();

    let folders = folder_list
        .iter()
        .map(|f| f.name().to_string())
        .collect::<Vec<_>>();

    folders.into()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![imap_get_folders]
}
