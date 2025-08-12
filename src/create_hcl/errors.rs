#[derive(Debug)]
pub enum CreateHCLError {
    InputError(String),
    NotFoundError(String),
}
