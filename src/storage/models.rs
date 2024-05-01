use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    filename: String,
    tags: Vec<String>,
    source_url: String,
}
