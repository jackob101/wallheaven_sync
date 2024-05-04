use core::panic;

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
