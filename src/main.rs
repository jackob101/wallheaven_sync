use std::{env, process::exit};

use storage::models::Metadata;
use wallheaven::models::Wallpaper;

mod prompts;
mod storage;
mod wallheaven;
fn main() {
    let username = match env::args().nth(1) {
        Some(val) => val,
        None => prompts::get_string("Username"),
    };

    let storage_path = storage::get_storage_path();

    println!(
        "Storage path: {}",
        &storage_path
            .to_str()
            .expect("Failed to convert path to str")
    );

    if !storage_path.exists() {
        match prompts::get_input_bool("Storage doesn't exists, do You want to create it?") {
            true => storage::init_storage(&storage_path),
            false => {
                println!("Aborting");
                exit(0)
            }
        }
    }

    let collections = wallheaven::get_collections(&username);

    if collections.is_empty() {
        println!("There are no collections for user: {}", username);
        exit(0)
    }

    let selected_collection = prompts::select_collection(&collections);

    prompts::synchronization_info(selected_collection);

    let wallpapers_in_collection =
        wallheaven::get_wallpapers_from_collection(&username, selected_collection.id);

    let collection_from_storage =
        match storage::get_collection(&storage_path, &selected_collection.label) {
            Some(val) => val,
            None => {
                storage::init_collection(&storage_path, &selected_collection.label);
                vec![]
            }
        };

    let not_synced = find_not_synced(&wallpapers_in_collection, &collection_from_storage);

    if not_synced.is_empty() {
        prompts::info("Everything is up to date");
        exit(0);
    }

    prompts::info_print("Wallpapers to sync", &not_synced, |e| &e.url);

    let confirm_sync = prompts::get_input_bool_with_default("Do you want to continue?", true);

    if !confirm_sync {
        println!("Aborting");
        exit(0);
    }

    let mut new_metadata = vec![];

    println!("Downloading:");

    for (index, e) in not_synced.iter().enumerate() {
        println!("[{}/{}] {}...", index + 1, not_synced.len(), e.url);
        let file_metadata = wallheaven::download_wallpaper_metadata(&e);
        //TODO split this. wallheaven module should download the file bytes and the storage module
        //should save it into hard drive
        wallheaven::save_image_content(
            &file_metadata.source_url,
            &storage_path,
            &selected_collection.label,
            &file_metadata.filename,
        );

        new_metadata.push(file_metadata);
    }

    let updated_collection: Vec<Metadata> = collection_from_storage
        .into_iter()
        .chain(new_metadata.into_iter())
        .collect();

    storage::persist_metadata(
        updated_collection,
        &selected_collection.label,
        &storage_path,
    )
}

fn find_not_synced<'a>(
    from_wallheaven: &'a Vec<Wallpaper>,
    from_collection: &Vec<Metadata>,
) -> Vec<&'a Wallpaper> {
    let mut not_synced: Vec<&Wallpaper> = vec![];
    for x in from_wallheaven {
        let mut contains = false;
        for y in from_collection {
            if *x.url == *y.source_url {
                contains = true;
                break;
            }
        }
        if !contains {
            not_synced.push(x);
        }
    }

    not_synced
}
