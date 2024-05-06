use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub filename: String,
    pub tags: Vec<String>,
    pub source_url: String,
}
