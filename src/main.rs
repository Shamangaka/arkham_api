mod api;
mod models;

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
        "update" => println!("Updating..."),
        _ => println!("Invalid command"),
    }
}

fn init() {
    println!("Initializing...");

    // get full card list using the API
    let _cards = api::init();

    // print card list
    println!("Card list: {:?}", _cards);
}
