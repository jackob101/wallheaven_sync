use std::{env, fs, path::PathBuf, process::exit};

use reqwest::Url;
use storage::models::Metadata;
use wallheaven::models::Wallpaper;

mod prompts;
mod storage;
mod wallheaven;
mod webclient;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--refresh" => refresh(),
            "--rebuild" => rebuild(),
            username => sync(username),
        }
    } else {
        sync(&prompts::get_string("Username"));
    }
}

fn rebuild() {
    let storage_path = storage::get_storage_path();

    if !storage_path.exists() {
        println!("Cannot refresh not existing collection");
        exit(1);
    }

    let collections = storage::get_collections(&storage_path);

    if collections.is_empty() {
        prompts::info("There are no collections in storage");
        exit(0)
    }

    let selection = prompts::select_from_list("Collections", &collections, |e| &e);

    let collection_path = storage_path.join(selection);

    let wallpapers = match storage::get_collection(&storage_path, selection) {
        Some(value) if !value.is_empty() => value,
        Some(_) => {
            prompts::info("Collection is empty");
            exit(0)
        }
        None => {
            println!("Nothing to refresh");
            exit(0)
        }
    };

    let files = read_filenames_from_directory(&collection_path);

    let to_rebuild = wallpapers
        .iter()
        .filter(|e| !files.iter().any(|f| f.eq(&e.filename)))
        .collect::<Vec<&Metadata>>();

    prompts::info_print("Wallpapers to redownload", &to_rebuild, |e| &e.filename);
    prompts::info("Downloading:");

    let to_rebuild_size = to_rebuild.len();

    for (index, e) in to_rebuild.iter().enumerate() {
        prompts::print_progress(index + 1, to_rebuild_size, &e.filename);
        let response = match webclient::download_image(Url::parse(&e.image_url).unwrap()) {
            Ok(response) => response,
            Err(err) => {
                println!("Failed to download image: {}", err);
                continue;
            }
        };

        let file_path = collection_path.join(&e.filename);

        if let Err(err) = fs::write(file_path, response) {
            println!("Failed to write to file {}", err);
        }
    }
}

fn refresh() {
    let storage_path = storage::get_storage_path();

    if !storage_path.exists() {
        println!("Cannot refresh not existing collection");
        exit(1);
    }

    let collections = storage::get_collections(&storage_path);

    if collections.is_empty() {
        prompts::info("There are no collections in storage");
        exit(0)
    }

    let selection = prompts::select_from_list("Collections", &collections, |e| &e);
    let wallpapers = match storage::get_collection(&storage_path, selection) {
        Some(value) => value,
        None => {
            println!("Nothing to refresh");
            exit(0)
        }
    };

    let collection_path = storage_path.join(selection);
    let files = read_filenames_from_directory(&collection_path);

    for file in &files {
        match wallpapers.iter().any(|e| e.filename.eq(file)) {
            false => {
                let _ = fs::remove_file(collection_path.join(&file));
                ()
            }
            true => (),
        };
    }

    let updated_metadata: Vec<Metadata> = wallpapers
        .into_iter()
        .filter(|e| files.contains(&e.filename))
        .collect();

    storage::persist_metadata(updated_metadata, selection, &storage_path);
}

fn sync(username: &str) {
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

    let not_synced_len = not_synced.len();

    for (index, e) in not_synced.iter().enumerate() {
        prompts::print_progress(index + 1, not_synced_len, &e.url);
        let file_metadata = wallheaven::download_wallpaper_metadata(&e);
        //TODO split this. wallheaven module should download the file bytes and the storage module
        //should save it into hard drive
        wallheaven::save_image_content(
            &file_metadata.image_url,
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

fn read_filenames_from_directory(path: &PathBuf) -> Vec<String> {
    fs::read_dir(path)
        .unwrap()
        .into_iter()
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap())
        .map(|e| e.file_name())
        .map(|e| e.to_str().unwrap().to_owned())
        .filter(|e| !(*e).eq("index.json"))
        .collect()
}
