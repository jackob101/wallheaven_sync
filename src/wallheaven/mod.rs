use core::panic;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use uuid::Uuid;

use crate::storage::models::Metadata;

use self::models::Wallpaper;

use self::models::Collection;

pub mod models;

pub fn get_collections(username: &str) -> Vec<Collection> {
    let url = format!("https://wallhaven.cc/api/v1/collections/{}", username);
    let body = reqwest::blocking::get(&url)
        .expect("Failed to fetch collections for user")
        .text()
        .unwrap();

    match serde_json::from_str::<models::CollectionsResponse>(&body) {
        Ok(value) => value.data,
        Err(_) => match serde_json::from_str::<models::CollectionsErrorResponse>(&body) {
            Ok(value) => match value.error.as_str() {
                "Nothing here" => vec![],
                value => panic!("Unhandled error response: {}", value),
            },
            Err(err) => panic!("failed to parse wallheaven API response: {}", err),
        },
    }
}

pub fn get_wallpapers_from_collection(username: &str, collection_id: i32) -> Vec<Wallpaper> {
    let mut page = 1;

    let mut wallpapers: Vec<Wallpaper> = vec![];

    loop {
        let url = format!(
            "https://wallhaven.cc/api/v1/collections/{}/{}?page={}",
            username, collection_id, page
        );

        let body = reqwest::blocking::get(&url).unwrap().text().unwrap();

        let mut response: models::CollectionWallpapersResponse =
            serde_json::from_str(&body).unwrap();

        wallpapers.append(&mut response.data);

        if response.meta.last_page < page {
            break;
        }

        page += 1;
    }

    wallpapers
}

pub fn download_wallpaper_metadata(wallpaper: &Wallpaper) -> Metadata {
    let url = format!("https://wallhaven.cc/api/v1/w/{}", &wallpaper.id);
    let body = reqwest::blocking::get(&url).unwrap().text().unwrap();

    let response: models::WallpaperDetailsResponse = serde_json::from_str(&body).unwrap();
    let original_thumb = response.data.thumbs.original;
    let image_extension = match original_thumb.split(".").last() {
        Some(extension) => extension,
        None => "jpg",
    };
    let uuid = uuid::Uuid::new_v4();

    Metadata {
        filename: format!("{}.{}", uuid.to_string(), image_extension),
        tags: response.data.tags.into_iter().map(|e| e.name).collect(),
        source_url: wallpaper.url.clone(),
    }
}

pub fn save_image_content(
    url: &str,
    storage_path: &PathBuf,
    collection_name: &str,
    filename: &str,
) {
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let full_path = storage_path.join(collection_name).join(filename);

    let mut file = match File::create(full_path) {
        Ok(file) => file,
        Err(err) => {
            panic!("{}", err)
        }
    };

    let _ = file.write_all(body.as_bytes());
}
