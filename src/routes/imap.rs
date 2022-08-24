use crate::utils::imap::{connect, ConnectOptions};
use rocket::serde::json::Json;

#[get("/imap/folders?<host>&<port>&<starttls>&<username>&<password>", format = "any")]
fn imap_get_folders(host: String, port: u16, starttls: bool, username: String, password: String) -> Json<Vec<String>> {
    let mut session = connect(ConnectOptions {
        host,
        port,
        username,
        password,
        starttls,
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
