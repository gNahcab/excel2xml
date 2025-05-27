use std::env::VarError;

#[derive(Debug)]
pub enum  APICallError {
    EnvError(VarError)
}
impl From<VarError> for APICallError {
    fn from(error: VarError) -> Self {
        APICallError::EnvError(error)
    }
}
