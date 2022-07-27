mod imap;
mod index;

use crate::models::response::Response;
use crate::routes;

use rocket::{
    http::{ContentType, Status},
    local::blocking::Client,
};
