use std::{env, process::exit};

mod prompts;
mod storage;
mod wallheaven;
fn main() {
    let username = match env::args().nth(1) {
        Some(val) => val,
        None => prompts::get_string("Username"),
    };

    let collections = wallheaven::get_collections(&username);

    if collections.is_empty() {
        println!("There are no collections for user: {}", username);
        exit(0)
    }

    let selected_collection = prompts::select_collection(&collections);

    prompts::synchronization_info(selected_collection);

    let wallpapers_in_collection =
        wallheaven::get_wallpapers_from_collection(&username, selected_collection.id);

    println!(
        "There are {} wallpapers in collection",
        wallpapers_in_collection.len()
    );

    let storage_path = storage::get_storage_path();

    if !storage_path.exists() {
        match prompts::get_input_bool("Storage doesn't exists, do You want to create it?") {
            true => storage::init_storage(&storage_path),
            false => {
                println!("Aborting");
                exit(0)
            }
        }
    }

    println!(
        "Storage under path {}",
        &storage_path
            .to_str()
            .expect("Failed to convert path to str")
    );

    let collection = match storage::get_collection(&storage_path, &selected_collection.label) {
        Some(val) => val,
        None => {
            storage::init_collection(&storage_path, &selected_collection.label);
            vec![]
        }
    };
}
