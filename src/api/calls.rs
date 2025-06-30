use std::collections::HashMap;
use serde_json::Value;
use crate::api::error::APICallError;
use std::env;
use std::fmt::Debug;
use std::io::Read;
use clap::builder::TypedValueParser;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};


pub async fn call_for_resources(token: &String, shortcode: &String, server: &String) -> Result<String, APICallError> {
    // /v2/metadata/projects/{projectShortcode}/resources
    // https://api.dasch.swiss/v2/authentication
    let address = format!("https://{}/v2/metadata/projects/{}/resources", server, shortcode);
    let response = reqwest::Client::new()
        .get(address)
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .send().await?;
    let text = response.text().await?;
    Ok(text)
}
pub async fn receive_token(email: String, password: String, server: &String) -> Result<String, APICallError> {
    let json_data= format!(r#"{{"email":"{}", "password":"{}"}}"#, email, password);
    // https://api.dasch.swiss/v2/authentication
   let address = format!("https://{}/v2/authentication", server);
   let response = reqwest::Client::new()
        .post(address)
       .header("Content-Type", "application/json")
       .body(json_data)
       .send().await?;

    let response_body = response.json::<HashMap<String, String>>().await?;
    match response_body.get("token") {
        None => {
            Err(APICallError::ContentError(format!("Response should return token, but wasn't found: '{:?}'", response_body)))
        }
        Some(token) => {
            println!("token received.");
            Ok(token.to_owned())
        }
    }
}

pub async fn delete_token(token: &String, server: String) -> Result<(), APICallError> {
    let json_data= format!(r#"{{"delete":"{}"}}"#, token);
    // https://api.dasch.swiss/v2/authentication
    let address = format!("https://{}/v2/authentication", server);
    let response = reqwest::Client::new()
        .get(address)
        .header("Content-Type", "application/json")
        .bearer_auth(&token)
        .body(json_data)
        .send().await?;
    if !&response.status().is_success() {
        let text = response.text().await;
        return Err(APICallError::NoSuccess(format!("Couldn't delete token '{}', message was: '{:?}'", token, text)))
    }
    Ok(())
}

pub fn import_email_pw_from_env() -> Result<(String, String), APICallError> {
    dotenv().ok();
    let pw_val = env::var("PASSWORD")?;
    let email_val = env::var("EMAIL")?;
    println!("USING EMAIL: {}, PASSWORD: {}", email_val, pw_val);
    Ok((email_val, pw_val))
}

