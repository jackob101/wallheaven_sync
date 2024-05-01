use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionsResponse {
    pub data: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub id: i32,
    pub label: String,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionWallpapersResponse {
    pub meta: Meta,
    pub data: Vec<Wallpaper>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallpaper {
    pub url: String,
    pub path: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub last_page: i32,
}
