use hcl::Number;
use crate::parse_info::errors::HCLDataError;

pub trait NumberTrait {
    fn as_usize(&self) -> Result<usize, HCLDataError>;
    fn as_u8(&self) -> Result<u8, HCLDataError>;

    fn as_f64_2(&self) -> Result<f64, HCLDataError>;
}

impl NumberTrait for hcl::Number {

    fn as_usize(&self) -> Result<usize, HCLDataError> {
        let number = self.as_f64_2()?;
        let usize_result = number as usize;
        Ok(usize_result)
    }
    fn as_u8(&self) -> Result<u8, HCLDataError> {
        let number = self.as_f64_2()?;
        let u8result = number.floor() as u8;
        Ok(u8result)
    }
    fn as_f64_2(&self) -> Result<f64, HCLDataError> {
        match self.as_f64() {
            None => {
                Err(HCLDataError::ParsingError(format!("couldn't parse this number '{}' to f64.", self)))
            }
            Some(number) => {
                Ok(number)
            }
        }
    }
}