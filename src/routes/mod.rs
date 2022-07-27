pub mod imap;
pub mod index;

pub fn routes() -> Vec<rocket::Route> {
    [index::routes(), imap::routes()].concat()
}
