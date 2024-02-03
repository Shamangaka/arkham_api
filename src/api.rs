use serde_json;

const BASE_URL: &str = "https://arkhamdb.com/api/public/";

#[tokio::main]
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    // replace this for all cards after testing
    let url = format!("{}{}", BASE_URL, "cards/core.json");
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let json = response.json::<serde_json::Value>().await?;
        organize_cards(json);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

fn organize_cards(json: serde_json::Value) {
    let formatted_json = serde_json::to_string_pretty(&json);
    
    // match formatted_json.type_code.as_str() {
    //     "act" => println!("act"),
    //     "adventure" => println!("adventure"),
    //     "agenda" => println!("agenda"),
    //     "asset" => println!("asset"),
    //     "enemy" => println!("enemy"),
    //     "event" => println!("event"),
    //     "investigator" => println!("investigator"),
    //     "key" => println!("key"),
    //     "location" => println!("location"),
    //     "scenario" => println!("scenario"),
    //     "skill" => println!("skill"),
    //     "story" => println!("story"),
    //     "treachery" => println!("treachery"),
    //     _ => println!("UNOWNED TYPE CODE: {}", formatted_json.type_code.as_str()),
    // } 
    // std::fs::write("core.json", formatted_json)?;

    // let investigator: models::Investigator = response.json().await?;

    // save json to file
    // let json = serde_json::to_string(&investigator).unwrap();
    // std::fs::write("investigator.json", json).unwrap();

    // println!("{:?}", investigator);
}
