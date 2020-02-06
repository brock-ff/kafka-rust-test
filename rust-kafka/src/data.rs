use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Formatter;

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub name: String,
    pub id: u64,
}

impl Data {
    pub fn new() -> Self {
        // get random ID number
        let mut rng = rand::thread_rng();
        let id: u64 = rng.gen_range(69, 420);
        Data {
            name: format!("item {}", id),
            id,
        }
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&serde_json::to_string_pretty(self).unwrap())
    }
}
