use std::collections::HashMap;
use serde_json::Value;
use crate::api::error::APICallError;
use std::env;
use std::fmt::Debug;
use std::io::Read;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
struct Auth {
    headers: HashMap<String, String>,
}

#[tokio::main]
async fn get_token(email: String, password: String, server: String) -> Result<String, APICallError> {
    let json_data= format!(r#"{{"email":"{}", "password":"{}"}}"#, email, password);
    // https://api.dasch.swiss/v2/authentication
   let address = format!("https://{}/v2/authentication", server);
   let send_result = reqwest::Client::new()
        .post(address)
       .header("Content-Type", "application/json")
       .body(json_data).send().await;
    let response = match send_result {
        Ok(response) => {
            response
        }
        Err(err) => {
         //   return Err();
            panic!("not implemented err")
        }
    };

    println!("response: {:?}", response);

    todo!()







}
fn json_contains_useful_data(value: &serde_json::Value) -> bool {
    match value {
        Value::Null => {
            false
        }
        Value::Bool(_) => {
            false
        }
        Value::Number(_) => {
            false
        }
        Value::String(_) => {
            false
        }
        Value::Array(_) => {
            false
        }
        Value::Object(object) => {
            !object.is_empty()
        }
    }
}

fn import_email_pw_from_env() -> Result<(String, String), APICallError> {
    dotenv().ok();
    let pw_val = env::var("PASSWORD")?;
    let email_val = env::var("EMAIL")?;
    println!("USING EMAIL: {}, PASSWORD: {}", email_val, pw_val);
    Ok((email_val, pw_val))
}

mod test {
    use std::fmt::Debug;

    #[test]
    fn test_api() {
        todo!()
    }
}
