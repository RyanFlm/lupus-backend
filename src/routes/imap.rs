use crate::{
    models::response::Response,
    utils::imap::{connect, ConnectOptions},
};
use rocket::serde::json::{serde_json::json, Json};
use utf7_imap::decode_utf7_imap;

#[get("/imap", format = "any")]
fn imap_index() -> Json<Response> {
    Json(Response {
        success: true,
        message: "Welcome to the imap path".to_string(),
        data: json!(()),
    })
}

#[get("/imap/folders", format = "any")]
fn imap_folders() -> Json<Response> {
    Json(Response {
        success: true,
        message: "Welcome to the imap/folders path".to_string(),
        data: json!(()),
    })
}

#[get(
    "/imap/folders?<host>&<port>&<starttls>&<username>&<password>",
    format = "any"
)]
fn imap_get_folders(
    host: String,
    port: u16,
    starttls: bool,
    username: String,
    password: String,
) -> Json<Response> {
    let mut session = connect(ConnectOptions {
        host,
        port,
        username,
        password,
        starttls,
    })
    .expect("Failed to connect to IMAP server");

    let folder_list = session.list(None, Some("%")).unwrap();

    let mut folders = folder_list
        .iter()
        .map(|f| decode_utf7_imap(f.name().to_string()))
        .collect::<Vec<_>>();

    folders.sort(); // Order alphabetically

    Json(Response {
        success: true,
        message: "".to_string(),
        data: json!(folders),
    })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![imap_index, imap_folders, imap_get_folders]
}
