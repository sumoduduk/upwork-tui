use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod core_opt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JobPost {
    pub title: String,
    pub link: String,
    pub category: String,
    pub detail: HashMap<String, String>,
    pub posted_on: String,
    pub posted_timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct FinalPost {
    title: String,
    link: String,
    detail: String,
    price: String,
}
