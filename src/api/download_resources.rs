use crate::api::calls::{receive_token, import_email_pw_from_env, call_for_resources, delete_token};
use crate::api::error::APICallError;

#[tokio::main]
pub async fn metadata_download(server: String, shortcode: &String) -> Result<String, APICallError> {
    let (email, pw) = import_email_pw_from_env()?;
    let token = receive_token(email, pw, &server).await?;
    let resources_text_csv = call_for_resources(&token, shortcode, &server).await?;
    delete_token(&token, server).await?;
    Ok(resources_text_csv)
}


