use crate::utils::imap;

#[get("/imap/folders")]
fn imap_get_folders() {
    // Load folders from IMAP server
    imap::test();
}

pub fn routes() -> Vec<rocket::Route> {
    routes![imap_get_folders]
}
