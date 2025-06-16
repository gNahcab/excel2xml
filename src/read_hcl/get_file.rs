use std::path::Path;
use crate::read_hcl::errors::ReadHCLError;

pub fn read_hcl_body<P: AsRef<Path>>(path: P) -> Result<hcl::Body, ReadHCLError> {
    let input = std::fs::read_to_string(path);
    let inputstr = match input {
        Ok(str_) => str_,
        Err(error) =>
            return Err(ReadHCLError::IO(error))
        ,
    };
    let body:hcl::Body = hcl::from_str(&inputstr)?;
    Ok(body)
}
