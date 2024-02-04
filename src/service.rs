use crate::models;
use serde_json::{Error, Value};
use std::env;
use std::fs;
use std::path::PathBuf;

trait CardHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error>;
}

struct DefaultHander;
impl CardHandler for DefaultHander {
    fn handle_card(&self, _card: Value) -> Result<(), Error> {
        println!("Unknown card type");
        Ok(())
    }
}

struct ActHandler;
impl CardHandler for ActHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let act: models::act::Act = serde_json::from_value(card.clone())?;

        let path = create_card_file(card);
        let act_value = act.to_string_pretty();
        save_card_to_file(path, act_value?);
        Ok(())
    }
}

fn get_card_handler(type_code: &str) -> Box<dyn CardHandler> {
    match type_code {
        "act" => Box::new(ActHandler {}),
        _ => Box::new(DefaultHander),
    }
}

pub fn categorize_cards(resp: String) {
    let json = serde_json::from_str(&resp).unwrap();

    if let Value::Array(cards) = json {
        for card in cards {
            if let Some(type_code) = card.get("type_code").and_then(|tc| tc.as_str()) {
                let handler = get_card_handler(type_code);
                match handler.handle_card(card) {
                    Ok(_) => {}
                    Err(e) => println!("Error handling card: {:?}", e),
                }
            };
        }
    }
}

fn create_card_file(card: Value) -> PathBuf {
    let pack_code = card.get("pack_code").and_then(|pc| pc.as_str()).unwrap();
    let code = card.get("code").and_then(|c| c.as_str()).unwrap();
    let type_code = card.get("type_code").and_then(|tc| tc.as_str()).unwrap();

    let path_components = ["data", pack_code, type_code, code];

    let mut file_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    file_path.extend(&path_components);
    file_path.set_extension("json");

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directories");
    };

    file_path
}

fn save_card_to_file(path: PathBuf, contents: String) {
    match fs::write(path.clone(), contents) {
        Ok(_) => println!("Wrote to {:?}", path),
        Err(e) => println!("Error writing to {:?}: {:?}", path, e),
    }
}
