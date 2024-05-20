use std::{env, fs, path::PathBuf, process::exit};

use reqwest::Url;
use storage::models::Metadata;
use uuid::Uuid;
use wallheaven::models::Wallpaper;

//TODO: Clean code below, Add some proper handling to errors
//TODO: Add help menu
//TODO: The methods which send request in wallheaven modules should go to webclient module
//TODO: Change webclient module name to something better
//
mod prompts;
mod storage;
mod wallheaven;
mod webclient;
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => sync(),
        2 => match args[1].as_str() {
            "--refresh" => refresh(),
            "--rebuild" => rebuild(),
            "--add" => add(),
            "--help" => help(),
            _ => sync(),
        },
        _ => help(),
    };
}

fn help() {
    println!(
        "\
Usage: wallheaven_sync [OPTION] 
       wallheaven_sync [USERNAME] 

Options: 
--refresh      Do not use
--rebuild      Download all wallpapers declared in index.json
--add          Add new wallpaper to index.json
--help         Print help menu"
    )
}

fn add() {
    let storage_path = storage::get_storage_path();

    if !storage_path.exists() {
        println!("Storage directory is missing");
        exit(1);
    }

    let collections = storage::get_collections(&storage_path);

    if collections.is_empty() {
        prompts::info("There are no collections in storage");
        exit(0)
    }

    let selection = prompts::select_from_list("Collections", &collections, |e| &e);

    let mut collection = storage::get_collection(&storage_path, selection).unwrap();
    let url = get_url();

    if collection.iter().any(|e| e.image_url.eq(&url.to_string())) {
        prompts::info("Url already exists in storage metadata, aborting");
        exit(1);
    };

    let tags = get_tags();
    let extension = get_extension(&url);
    let filename = format!("{}.{}", Uuid::new_v4().to_string(), extension);

    let metadata = Metadata {
        filename,
        tags,
        source_url: url.to_string(),
        image_url: url.to_string(),
    };

    match webclient::download_image(&url) {
        Ok(value) => {
            storage::save(&storage_path, selection, &metadata.filename, value);
            prompts::info(&format!("Saved {}", &metadata.filename));
            collection.push(metadata);
            storage::persist_metadata(collection, selection, &storage_path);
        }
        Err(_) => {
            println!("Failed to download image");
            exit(1);
        }
    }
}

fn get_url() -> Url {
    let url = prompts::get_input_string("Url");
    match Url::parse(&url) {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid value, please try again");
            get_url()
        }
    }
}

fn get_tags() -> Vec<String> {
    let value = prompts::get_input_string("Provide tags ( separated by , )");

    value
        .split(",")
        .map(|e| e.to_owned())
        .collect::<Vec<String>>()
}

fn get_extension(url: &Url) -> String {
    match url.to_string().split(".").last() {
        Some(value) if !value.is_empty() => {
            let answer = prompts::get_input_string(&format!(
                "Filetype {}, input to override or leave blank to confirm",
                value
            ));
            if answer.is_empty() {
                value.to_owned()
            } else {
                answer
            }
        }
        _ => prompts::get_input_string("Input file extension ( example: jpg )"),
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
        let response = match webclient::download_image(&Url::parse(&e.image_url).unwrap()) {
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

fn sync() {
    let username = &prompts::get_input_string("Username");

    let storage_path = storage::get_storage_path();

    println!(
        "Storage path: {}",
        &storage_path
            .to_str()
            .expect("Failed to convert path to str")
    );

    if !storage_path.exists() {
        match prompts::get_input(
            "Storage doesn't exists, do You want to create it?[Y/n]",
            prompts::mappers::bool_mapper_with_default(true),
        ) {
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

    let selected_collection = prompts::select_from_list("Collections:", &collections, |e| &e.label);

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

    let confirm_sync = prompts::get_input(
        "Do you want to continue?[Y/n]",
        prompts::mappers::bool_mapper_with_default(true),
    );

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
