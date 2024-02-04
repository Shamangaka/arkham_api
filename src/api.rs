// use crate::service;
// use serde_json;
use std::fs;

// const BASE_URL: &str = "https://arkhamdb.com/api/public/";

// #[tokio::main]
// // TODO: While testing, load from local file instead of API
// pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
//     // replace this for all cards after testing
//     let url = format!("{}{}", BASE_URL, "cards/core.json");
//     let response = reqwest::get(&url).await?;

//     if response.status().is_success() {
//         let json = response.json::<serde_json::Value>().await?;
//         categorize_cards(json);
//     } else {
//         println!("Request failed with status: {}", response.status());
//     }

//     Ok(())
// }

pub fn init() -> Result<String, Box<dyn std::error::Error>> {
    let resp = fs::read_to_string("core.json").expect("Unable to read core.json");

    Ok(resp)
}
