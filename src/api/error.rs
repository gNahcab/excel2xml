use std::env::VarError;
use reqwest::Error;

#[derive(Debug)]
pub enum  APICallError {
    EnvError(VarError),
    ReqwestError(Error),
    ContentError(String),
    NoSuccess(String),
}
impl From<reqwest::Error> for APICallError {
    fn from(error: reqwest::Error) -> Self {
        APICallError::ReqwestError(error)
    }
}
impl From<VarError> for APICallError {
    fn from(error: VarError) -> Self {
        APICallError::EnvError(error)
    }
}
