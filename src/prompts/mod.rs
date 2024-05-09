use std::{
    io::{self, stdout, Write},
    str::FromStr,
};

use crate::wallheaven::models::Collection;

pub mod mappers;
pub mod validations;

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

    let selection = get_input_with_validation(
        "Select option",
        mappers::i32_mapper,
        validations::in_range(1, entries.len() as i32),
    ) - 1;

    entries
        .get(selection as usize)
        .expect("get_input_i32 guarantes value in range")
}

pub fn synchronization_info(collection: &Collection) {
    println!("Synchronizing collection: {}", collection.label)
}

pub fn get_input<T>(prompt: &str, mapper: impl Fn(&str) -> Option<T>) -> T
where
    T: FromStr,
{
    print!("{}: ", prompt);
    stdout().flush().expect("Failed to flush stdout!");
    let mut input = "".to_owned();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    match mapper(input.trim()) {
        Some(value) => value,
        None => {
            println!("Incorrect value! Please ");
            get_input(prompt, mapper)
        }
    }
}

pub fn get_input_with_validation<T>(
    prompt: &str,
    mapper: impl Fn(&str) -> Option<T>,
    validation: impl Fn(&T) -> Option<String>,
) -> T
where
    T: FromStr,
{
    let value = get_input(prompt, &mapper);

    match validation(&value) {
        Some(error) => {
            println!("{}", error);
            get_input_with_validation(prompt, mapper, validation)
        }
        None => value,
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
