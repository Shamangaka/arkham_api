mod api;
mod models;
mod service;

use std::io;

fn main() {
    println!("Enter a command <init|update>");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    println!("You entered: {}", input);

    match input {
        "init" => init(),
        "update" => update(),
        _ => println!("Invalid command"),
    }
}

fn init() {
    println!("Initializing...");

    api::init();
}

fn update() {
    println!("Update not implemented yet.");
    // println!("Updating...");
}
