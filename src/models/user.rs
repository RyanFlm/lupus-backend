use rocket::{serde::{uuid::Uuid}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub age: u8,
}


impl User {
    pub fn load(id: Uuid) -> Self {
        // Load user from database
        User {
            id,
            name: "John Doe".to_string(),
            age: 42,
        }
    }
}
