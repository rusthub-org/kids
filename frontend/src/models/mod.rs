pub mod home;
pub mod categories;
pub mod topics;
pub mod projects;
pub mod users;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Page {
    pub from: i64,
    pub first: String,
    pub last: String,
}

impl Default for Page {
    fn default() -> Self {
        Self { from: 1, first: String::from("-"), last: String::from("-") }
    }
}
