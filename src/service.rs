use crate::models;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn categorize_cards(json: serde_json::Value) {
    if let serde_json::Value::Array(cards) = json {
        for card in cards {
            if let Some(type_code) = card.get("type_code").and_then(|tc| tc.as_str()) {
                match type_code {
                    "act" => serialize_card("act", card),
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
        }
    };
}

fn serialize_card(type_code: &str, card: serde_json::Value) {
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
        let investigator: models::investigator::Investigator =
            serde_json::from_value(card).expect("Error parsing investigator");
        fs::write(
            &file_path,
            serde_json::to_string_pretty(&investigator).unwrap(),
        )
        .expect("Failed to write to file");
        println!("Wrote investigator to {:?}", file_path);
    } else if type_code == "act" {
        let act: models::act::Act = serde_json::from_value(card).expect("Error parsing act");
        fs::write(&file_path, serde_json::to_string_pretty(&act).unwrap())
            .expect("Failed to write to file");
        println!("Wrote act to {:?}", file_path);
    };
}
