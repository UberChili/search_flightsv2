use reqwest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let amad_api = env::var("AMAD_API_KEY").unwrap();
    let amad_secret = env::var("AMAD_SECRET").unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post("https://test.api.amadeus.com/v1/security/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "grant_type=client_credentials&client_id={}&client_secret={}",
            amad_api, amad_secret
        ))
        .send()
        .await?;

    println!("status: {}", res.status());
    let body = res.text().await?;
    println!("Response body: {}", body);

    Ok(())
}
