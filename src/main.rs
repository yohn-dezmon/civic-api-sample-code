use std::error::Error;
use reqwest::Client;
use std::env;

// tokio runtime for async code
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Make this an environment variable
    let api_key = env::var("OPEN_FEC_API_KEY").unwrap();
    // Daniel Goldman's candidate_id
    let candidate_id = "H2NY10308";
    let client = Client::new();
    let url = format!("https://api.open.fec.gov/v1/candidate/{}?api_key={}", candidate_id, api_key);
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}