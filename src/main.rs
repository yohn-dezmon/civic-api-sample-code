use hyper::{Client, Uri};
use std::str::FromStr;

// tokio runtime for async code
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // using &str because I want to be able to reuse `url` and I won't need to 
    // modify it.
    let host_name: &str = "https://api.open.fec.gov/v1";
    let endpoint: &str = "/candidate/1";

    let candidate_1_endpoint = format!("{}{}", host_name, endpoint);
    let c_1_ref = candidate_1_endpoint.as_str();
    
    let http_client = Client::new();
    // why is there a question mark before the semi-colon?
    let endpoint_uri = Uri::from_str(c_1_ref)?;
    let response = http_client.get(endpoint_uri).await?;
    let response_body = hyper::body::to_bytes(response.into_body()).await?;

    let response_string = String::from_utf8_lossy(&response_body);
    println!("{}", response_string);

    Ok(())
}
