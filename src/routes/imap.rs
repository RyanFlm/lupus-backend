use imap;
use rocket::serde::json::Json;
use rustls_connector::RustlsConnector;

#[get("/imap/folders?<domain>&<username>&<password>")]
fn imap_get_folders(domain: String, username: String, password: String) -> Json<Vec<String>> {
    let client = imap::ClientBuilder::new(domain, 143)
        .starttls()
        .connect(|domain, tcp| {
            let ssl_conn = RustlsConnector::new_with_native_certs()?;
            Ok(ssl_conn.connect(domain, tcp).unwrap())
        })
        .unwrap();
    let mut session = client.login(username, password).map_err(|e| e.0).unwrap();

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
