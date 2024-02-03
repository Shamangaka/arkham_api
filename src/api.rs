use crate::models::investigator::Investigator;
use serde_json;
use std::env;
use std::fs;
use std::path::PathBuf;

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

pub fn init() {
    let json = fs::read_to_string("core.json").expect("Unable to read core.json");
    // parse into json
    let json = serde_json::from_str(&json).expect("Unable to parse json");

    categorize_cards(json);
}

fn categorize_cards(json: serde_json::Value) {
    // let formatted_json = serde_json::to_string_pretty(&json);

    // Assuming json is an array of card objects
    if let serde_json::Value::Array(cards) = json {
        for card in cards {
            if let Some(type_code) = card.get("type_code").and_then(|tc| tc.as_str()) {
                match type_code {
                    "act" => println!("act"),
                    "adventure" => println!("adventure"),
                    "agenda" => println!("agenda"),
                    "asset" => println!("asset"),
                    "enemy" => println!("enemy"),
                    "event" => println!("event"),
                    "investigator" => serialize_card("investigator", card),
                    "key" => println!("key"),
                    "location" => println!("location"),
                    "scenario" => println!("scenario"),
                    "skill" => println!("skill"),
                    "story" => println!("story"),
                    "treachery" => println!("treachery"),
                    _ => println!("UNOWNED TYPE CODE"),
                };
            }
        };
    };
}

fn serialize_card(type_code: &str, card: serde_json::Value) {
    // get the set from the card
    let pack_code = card.get("pack_code").and_then(|pc| pc.as_str()).unwrap();
    let code = card.get("code").and_then(|c| c.as_str()).unwrap();

    let mut file_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    file_path.push("data");
    file_path.push(pack_code);
    file_path.push(type_code);
    file_path.push(code);
    file_path.set_extension("json");

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directories");
    };

    if type_code == "investigator" {
        let investigator: Investigator =
            serde_json::from_value(card).expect("Error parsing investigator");
        fs::write(
            &file_path,
            serde_json::to_string_pretty(&investigator).unwrap(),
        )
        .expect("Failed to write to file");
        println!("Wrote investigator to {:?}", file_path);
    };
}
