#[derive(Debug)]
pub enum WriteHCLError {
    InputError(String),
    NotFoundError(String),
}
