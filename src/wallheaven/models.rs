use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionsResponse {
    pub data: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionsErrorResponse {
    pub error: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct WallpaperDetailsResponse {
    pub data: WallpaperDetailsResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WallpaperDetailsResponseData {
    pub thumbs: Thumb,
    pub tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumb {
    pub original: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
}
