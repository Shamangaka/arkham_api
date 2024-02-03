use crate::models::investigator::Investigator;
use serde_json;

const BASE_URL: &str = "https://arkhamdb.com/api/public/";

#[tokio::main]
// TODO: While testing, load from local file instead of API
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    // replace this for all cards after testing
    let url = format!("{}{}", BASE_URL, "cards/core.json");
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let json = response.json::<serde_json::Value>().await?;
        categorize_cards(json);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
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
                }
            }
        }
    }

    // let investigator: models::Investigator = response.json().await?;

    //      save json to file
    //      let json = serde_json::to_string(&investigator).unwrap();
    //      std::fs::write("investigator.json", json).unwrap();

    //      println!("{:?}", investigator);
}

fn serialize_card(type_code: &str, card: serde_json::Value) {
    if type_code == "investigator" {
        let investigator: Investigator = serde_json::from_value(card).unwrap();
        println!("{:?}", investigator);
    }
}
