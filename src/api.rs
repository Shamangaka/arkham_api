// use serde_json::Value;
// use std::fs::File;
// use std::io::Write;
use std::fs;

// const BASE_URL: &str = "https://arkhamdb.com/api/public/";

#[tokio::main]
pub async fn init() -> Result<String, Box<dyn std::error::Error>> {
    let json_string = fs::read_to_string("cards_full.json").unwrap();

    // let url = format!("{}{}", BASE_URL, "cards/");

    // let client = reqwest::Client::new();

    // let response = client.get(&url).query(&[("encounter", "1")]).send().await?;

    // let mut json_string = String::new();

    // if response.status().is_success() {
    //     let cards: Value = response.json().await?;

    //     json_string = serde_json::to_string_pretty(&cards)?;

    //     let file_path = "cards_full.json";

    //     let mut file = File::create(file_path)?;
    //     file.write_all(json_string.as_bytes())?;

    //     // let json = response.json::<serde_json::Value>().await?;
    //     // categorize_cards(json);
    // } else {
    //     println!("Request failed with status: {}", response.status());
    // }

    Ok(json_string)
}

// pub fn init() -> Result<String, Box<dyn std::error::Error>> {
//     let resp = fs::read_to_string("core.json").expect("Unable to read core.json");

//     Ok(resp)
// }
