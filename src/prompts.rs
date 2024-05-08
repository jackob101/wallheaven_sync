use std::io::{self, stdout, Write};

use crate::wallheaven::{self, models::Collection};

pub fn select_collection(
    collections: &Vec<wallheaven::models::Collection>,
) -> &wallheaven::models::Collection {
    println!("Collections: ");

    collections
        .iter()
        .enumerate()
        .for_each(|(i, e)| println!("{}) {}", i + 1, e.label));

    let selection = get_input_i32("Select option", collections.len() as i32) - 1;

    collections
        .get(selection as usize)
        .expect("get_input_i32 guarantes value in range")
}

pub fn select_from_list<'a, F, T>(header: &str, entries: &'a Vec<T>, map: F) -> &'a T
where
    F: Fn(&T) -> &str,
{
    let body = entries
        .iter()
        .enumerate()
        .map(|(index, e)| format!("{} -> {}\n", index + 1, map(e)))
        .reduce(|acc, e| acc + &e)
        .expect("Vector must contains at least one element");

    println!("{}\n{}", header, body);
    let _ = stdout().flush();

    let selection = get_input_i32("Select option", entries.len() as i32) - 1;

    entries
        .get(selection as usize)
        .expect("get_input_i32 guarantes value in range")
}

pub fn synchronization_info(collection: &Collection) {
    println!("Synchronizing collection: {}", collection.label)
}

// pub fn get_input_string(prompt: &str) -> String {
//     print!("{}: ", prompt);
//     stdout().flush().expect("Failed to flush stdout!");
//     let mut input = "".to_owned();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read user input");
//     input = input.trim().to_owned();
//
//     match input.len() {
//         0 => {
//             println!("Please input value");
//             get_input_string(prompt)
//         }
//         _ => input,
//     }
// }

pub fn get_input_i32(prompt: &str, max_bound: i32) -> i32 {
    print!("{}: ", prompt);
    stdout().flush().expect("Failed to flush stdout!");
    let mut input = "".to_owned();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    match input.trim().parse::<i32>() {
        Ok(value) => {
            if value > max_bound || value <= 0 {
                println!("Value is out of range! Please try again");
                get_input_i32(prompt, max_bound)
            } else {
                value
            }
        }
        Err(_) => {
            println!("Incorrect value! Please ");
            get_input_i32(prompt, max_bound)
        }
    }
}

pub fn get_input_bool(prompt: &str) -> bool {
    print!("{}[y/n]: ", prompt);
    stdout().flush().expect("Failed to flush stdout!");
    let mut input = "".to_owned();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    match input.trim() {
        "y" | "Y" => true,
        "n" | "N" => false,
        _ => {
            println!("Invalid value, please try again");
            get_input_bool(prompt)
        }
    }
}

pub fn get_input_bool_with_default(prompt: &str, default: bool) -> bool {
    let options = match default {
        true => "[Y/n]",
        false => "[y/N]",
    };

    print!("{}{}: ", prompt, options);
    stdout().flush().expect("Failed to flush stdout!");
    let mut input = "".to_owned();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    match input.trim() {
        "y" | "Y" => true,
        "n" | "N" => false,
        "" => default,
        _ => {
            println!("Invalid value, please try again");
            get_input_bool(prompt)
        }
    }
}

pub fn info_print<F, T>(header: &str, new_metadata: &[T], mapper: F)
where
    F: Fn(&T) -> &str,
{
    let body = new_metadata
        .iter()
        .enumerate()
        .map(|(index, e)| format!("{} -> {}\n", index + 1, mapper(&e)))
        .reduce(|acc, e| acc + &e);

    match body {
        Some(value) => println!("{}:\n{}", header, value),
        None => (),
    }
}

pub(crate) fn info(prompt: &str) {
    println!("{}", prompt);
}

pub(crate) fn print_progress(index: usize, total: usize, body: &str) {
    println!("[{}/{}] {}...", index, total, body);
}

pub fn get_input_string(prompt: &str) -> String {
    print!("{}: ", prompt);
    stdout().flush().expect("Failed to flush stdout!");
    let mut input = "".to_owned();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    input.trim().to_owned()
}
