use core::panic;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct AccessToken {
    username: String,
    access_token: String,
}

// URL to send POST to and get access token
static URL: &str = "https://test.api.amadeus.com/v1/security/oauth2/token";

pub async fn get_access_token() -> Result<String, Box<dyn std::error::Error>> {
    // Getting env variables to obtain AMAD access token
    let amad_api = env::var("AMAD_API_KEY").unwrap_or_else(|error| {
        panic!("AMAD API Key not found in environment variables: {}", error)
    });
    let amad_secret = env::var("AMAD_API_SECRET").unwrap_or_else(|error| {
        panic!(
            "AMAD API Secret not found in environment variables: {}",
            error
        )
    });

    let client = reqwest::Client::new();
    let response = client
        .post(URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "grant_type=client_credentials&client_id={}&client_secret={}",
            amad_api, amad_secret
        ))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err("Failed to get access token: server returned error status".into());
    }

    let body = response.text().await?;
    let token: AccessToken = serde_json::from_str(&body)?;

    Ok(token.access_token)
}
