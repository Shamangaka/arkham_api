mod api;
mod models;
mod service;

use std::io;

fn main() {
    println!("Enter a command <init>");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    match input {
        "init" => init(),
        "update" => update(),
        _ => println!("Invalid command"),
    }
}

fn init() {
    println!("Initializing...");
    match api::init() {
        Ok(resp) => service::categorize_cards(resp),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn update() {
    println!("Update not implemented yet.");
    // println!("Updating...");
}
