mod prompts;
mod storage;
mod wallheaven;
fn main() {
    let collections = wallheaven::get_collections("TSear");

    let selected_collection = prompts::select_collection(&collections);

    prompts::synchronization_info(selected_collection);

    // wallheaven::get_wallpapers_from_collection("TSear", 1661827);
    //
    let storage_path = storage::get_storage_path();
}
