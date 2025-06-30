use crate::parse_hcl::errors::HCLDataError;

#[derive(Debug, PartialEq, Clone)]
pub enum DateType {
   Gregorian,
    Julian,
}
impl DateType {
    pub(crate) fn date_type(string: String) -> Result<DateType, HCLDataError>{
        match string.as_str() {
            "Gregorian" => Ok(DateType::Gregorian),
            "Julian" => Ok(DateType::Julian),
            _ => Err(HCLDataError::ParsingError(format!("unknown value for 'date'-attribute: {:?}", string))),
        }

    }
}
