use std::io::{self, stdout, Write};

use crate::wallheaven::{
    self,
    models::{self, Collection},
};

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

pub fn synchronization_info(collection: &Collection) {
    println!("Synchronizing collection: {}", collection.label)
}

fn get_input_i32(prompt: &str, max_bound: i32) -> i32 {
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
            println!("Incorrect value! Please input numeric value");
            get_input_i32(prompt, max_bound)
        }
    }
}
