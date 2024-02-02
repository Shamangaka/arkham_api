use serde_json::Value;

const BASE_URL: &str = "https://arkhamdb.com/api/public/";

#[tokio::main]
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}{}", BASE_URL, "card/01001");
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;

        // save json to file

        println!("{:?}", json);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(()) 
}
