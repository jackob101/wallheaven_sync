use self::models::Wallpaper;

use self::models::Collection;

pub mod models;

pub fn get_collections(username: &str) -> Vec<Collection> {
    let url = format!("https://wallhaven.cc/api/v1/collections/{}", username);
    let body = reqwest::blocking::get(&url)
        .expect("Failed to fetch collections for user")
        .text()
        .unwrap();

    let response: models::CollectionsResponse = serde_json::from_str(&body).unwrap();

    response.data
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
