use core::panic;
use std::{
    env,
    ffi::OsStr,
    fs::{self, read_dir, File},
    io::{Error, Write},
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

pub fn init_storage(storage_path: &PathBuf) {
    if let Err(err) = fs::create_dir(storage_path) {
        panic!("Failed to create storage directory: {}", err)
    }
}

pub fn init_collection(storage_path: &PathBuf, label: &str) {
    let collection_path = storage_path.join(label);

    if let Err(_) = fs::metadata(&collection_path) {
        match fs::create_dir(&collection_path) {
            Ok(val) => val,
            Err(err) => {
                panic!("Failed to create collection directory: {}", err)
            }
        };
    }
}

pub fn get_collection(storage_path: &PathBuf, label: &str) -> Option<Vec<Metadata>> {
    let collection_path = storage_path.join(label);

    if !Path::new(&collection_path).exists() {
        return None;
    }

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

    match metadata_file {
        Some(file) => match fs::read_to_string(file.path()) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(v) => v,
                Err(err) => panic!("Failed to parse index.json. {}", err),
            },
            Err(err) => {
                panic!("Failed to read content of index.json, {}", err);
            }
        },
        None => None,
    }
}

pub fn persist_metadata(updated_collection: Vec<Metadata>, label: &str, storage_path: &PathBuf) {
    let full_path_to_metadata = storage_path.join(label).join("index.json");
    let mut file = match File::create(full_path_to_metadata) {
        Ok(file) => file,
        Err(err) => {
            panic!("{}", err)
        }
    };

    let json = match serde_json::to_string(&updated_collection) {
        Ok(value) => value,
        Err(err) => panic!("{}", err),
    };

    let _ = file.write_all(json.as_bytes());
}
