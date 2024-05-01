use std::{
    env,
    ffi::OsStr,
    fs::{self, read_dir},
    path::{Path, PathBuf},
    str::FromStr,
};

use self::models::Metadata;

pub mod models;

const ALLOWED_WALLPAPER_FORMATS: [&'static str; 3] = ["jpg", "png", "jpeg"];

pub fn get_storage_path() -> PathBuf {
    let key = "WALLHEAVEN_SYNC_STORAGE_PATH";

    let value = match env::var(key) {
        Ok(value) => value,
        Err(_) => {
            //TODO write some better way to determine default path for windows
            let home = env::home_dir().unwrap().to_str().unwrap().to_owned();
            home + "/wallheaven_storage"
        }
    };

    PathBuf::from_str(&value).unwrap()
}

pub fn get_collection(storage_path: &PathBuf, label: &str) -> Vec<Metadata> {
    let collection_path = storage_path.join(label);

    let iterator = match read_dir(collection_path) {
        Ok(iterator) => iterator,
        Err(err) => {
            panic!("Failed to read collection directory: {}", err);
        }
    };

    let mut metadata_file = None;
    let mut entries = vec![];

    for e in iterator {
        let file = match e {
            Ok(file) => file,
            Err(err) => {
                println!("Skipping entry: {}", err);
                continue;
            }
        };

        if file.path().is_dir() {
            continue;
        }

        let filename = file.file_name();
        let file_format = Path::new(&filename).extension().and_then(OsStr::to_str);

        if "index.json" == filename {
            metadata_file = Some(file)
        } else if let Some(format) = file_format {
            if ALLOWED_WALLPAPER_FORMATS.contains(&format) {
                entries.push(file)
            }
        }
    }

    let metadata_content = match metadata_file {
        Some(file) => match fs::read_to_string(file.path()) {
            Ok(content) => content,
            Err(err) => {
                panic!("Failed to read content of index.json, {}", err);
            }
        },
        None => "".to_owned(),
    };

    match serde_json::from_str(&metadata_content) {
        Ok(v) => v,
        Err(err) => panic!("Failed to parse index.json. {}", err),
    }
}
