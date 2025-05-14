use hcl::Expression;
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::methods_domain::number_trait::NumberTrait;
#[derive(Debug, Clone, PartialEq)]
pub enum HeaderValue {
    Name(String),
    Number(u8)
}

impl HeaderValue {
    pub(crate) fn is_equal(&self, output: &String) -> bool {
        // return true if headerValue is equal to a string-value
        return match self {
            HeaderValue::Name(name) => {
                name == output
            }
            HeaderValue::Number(_) => {
                false
            }
        }
    }
}


pub trait HeaderMethods {
    fn to_header_value(&self) -> Result<HeaderValue, HCLDataError>;
}

impl HeaderMethods for hcl::Expression {
    fn to_header_value(&self) -> Result<HeaderValue, HCLDataError> {
        let header_value = match self {
            Expression::Number(number) => {
                HeaderValue::Number(number.as_u8()?)
            }
            Expression::String(string) => {
                HeaderValue::Name(string.to_owned())
            }
            _ => {
                return Err(HCLDataError::ParsingError(format!("Only transform Number and String-Expressions to HeaderValue, cannot transform this: '{:?}'", self)))
            }
        };
        Ok(header_value)
    }
}
