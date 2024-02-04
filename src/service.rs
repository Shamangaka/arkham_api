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
        println!("Unknown card type: {:?}", _card.get("type_code").unwrap());
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

struct AgendaHandler;
impl CardHandler for AgendaHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let agenda: models::agenda::Agenda = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let agenda_value = agenda.to_string_pretty();
        save_card_to_file(path, agenda_value?);
        Ok(())
    }
}

struct AssetHandler;
impl CardHandler for AssetHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let asset: models::asset::Asset = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let asset_value = asset.to_string_pretty();
        save_card_to_file(path, asset_value?);
        Ok(())
    }
}

struct EnemyHandler;
impl CardHandler for EnemyHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let enemy: models::enemy::Enemy = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let enemy_value = enemy.to_string_pretty();
        save_card_to_file(path, enemy_value?);
        Ok(())
    }
}

struct EventHandler;
impl CardHandler for EventHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let event: models::event::Event = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let event_value = event.to_string_pretty();
        save_card_to_file(path, event_value?);
        Ok(())
    }
}

struct InvestigatorHandler;
impl CardHandler for InvestigatorHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let investigator: models::investigator::Investigator =
            serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let investigator_value = investigator.to_string_pretty();
        save_card_to_file(path, investigator_value?);
        Ok(())
    }
}

struct LocationHandler;
impl CardHandler for LocationHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let location: models::location::Location = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let location_value = location.to_string_pretty();
        save_card_to_file(path, location_value?);
        Ok(())
    }
}

struct ScenarioHandler;
impl CardHandler for ScenarioHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let scenario: models::scenario::Scenario = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let scenario_value = scenario.to_string_pretty();
        save_card_to_file(path, scenario_value?);
        Ok(())
    }
}

struct SkillHandler;
impl CardHandler for SkillHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let skill: models::skill::Skill = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let skill_value = skill.to_string_pretty();
        save_card_to_file(path, skill_value?);
        Ok(())
    }
}

struct TreacheryHandler;
impl CardHandler for TreacheryHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let treachery: models::treachery::Treachery = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let treachery_value = treachery.to_string_pretty();
        save_card_to_file(path, treachery_value?);
        Ok(())
    }
}

fn get_card_handler(type_code: &str) -> Box<dyn CardHandler> {
    match type_code {
        "act" => Box::new(ActHandler {}),
        "agenda" => Box::new(AgendaHandler {}),
        "asset" => Box::new(AssetHandler {}),
        "enemy" => Box::new(EnemyHandler {}),
        "event" => Box::new(EventHandler {}),
        "investigator" => Box::new(InvestigatorHandler {}),
        "location" => Box::new(LocationHandler {}),
        "treachery" => Box::new(TreacheryHandler {}),
        "scenario" => Box::new(ScenarioHandler {}),
        "skill" => Box::new(SkillHandler {}),
        _ => Box::new(DefaultHander),
    }
}

pub fn categorize_cards(resp: String) {
    let json = serde_json::from_str(&resp).unwrap();

    if let Value::Array(cards) = json {
        for card in cards {
            if let Some(type_code) = card.get("type_code").and_then(|tc| tc.as_str()) {
                let code = card.get("code").and_then(|c| c.as_str()).unwrap();
                let handler = get_card_handler(type_code);
                match handler.handle_card(card.clone()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Handling card: {}", code);
                        println!("Type code: {}", type_code);

                        println!("Error handling card: {:?}", e);
                    }
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
        Ok(_) => {}
        Err(e) => println!("Error writing to {:?}: {:?}", path, e),
    }
}
