use crate::parse_dm::errors::DataModelError;

#[derive(Debug, PartialEq)]
pub enum Cardinality {
    ZeroToN,
    ZeroToOne,
    One,
    OneToN,
}

pub fn to_cardinality(cardinality: &str) -> Result<Cardinality, DataModelError> {
     match cardinality {
            "0-n" => {Ok(Cardinality::ZeroToN)}
            "0-1" => {Ok(Cardinality::ZeroToOne)}
            "1" => {Ok(Cardinality::One)}
         "1-n" => {Ok(Cardinality::OneToN)}
            _ => {Err(DataModelError::ParsingError(format!("Unknown cardinality: {:?}. Add first.", cardinality)))}
        }
}
