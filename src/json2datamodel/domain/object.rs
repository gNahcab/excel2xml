use crate::json2datamodel::domain::object::ValueObject::{BooleanValue, DateValue, GeonameValue, IntValue, ResLinkValue, ListValue, TextValue, TimeValue, UriValue, Representation, ColorValue, DecimalValue};
use crate::json2datamodel::errors::DataModelError;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValueObject {
    ListValue,
    TextValue,
    DateValue,
    UriValue,
    GeonameValue,
    DecimalValue,
    ColorValue,
    IntValue,
    BooleanValue,
    TimeValue,
    Representation,
    ResLinkValue(LinkElement),
}

pub struct ObjectWrapper (pub(crate) String);

impl ObjectWrapper {
    pub(crate) fn to_object(&self, onto_name: String) -> Result<ValueObject, DataModelError> {
        match self.0.as_str() {
            "TextValue"  => {
                Ok(TextValue)
            }
            "ListValue"  => {
                Ok(ListValue)
            }
            "DateValue"  => {
                Ok(DateValue)
            }
            "BooleanValue"  => {
                Ok(BooleanValue)
            }
            "GeonameValue"  => {
                Ok(GeonameValue)
            }
            "IntValue"  => {
                Ok(IntValue)
            }
            "TimeValue"  => {
                Ok(TimeValue)
            }
            "UriValue"  => {
                Ok(UriValue)
            }
            "ColorValue"  => {
                Ok(ColorValue)
            }
            "DecimalValue"  => {
                Ok(DecimalValue)
            }
            "Representation" => {
                Ok(Representation)
            }
            _ => {
                if self.0.contains(":") {
                    let link_element = LinkValueWrapper(self.0.to_string()).to_link_value(onto_name.to_owned())?;
                     Ok(ResLinkValue(link_element))

                } else {
                    Err(DataModelError::ParsingError(format!("this object-value: '{}' is unexpected. Either it is incorrect or not supported yet.", self.0)))
                }
            }
        }

    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LinkElement {
    // ontology is empty if it is current ontology
    ontology: String,
    resource: String,
}

struct LinkValueWrapper (String);
impl LinkValueWrapper {
    fn to_link_value(&self, onto_name: String) -> Result<LinkElement, DataModelError> {
        // one colon
        let chars = self.0.chars().collect::<Vec<_>>();
        let number_of_colons = chars.iter().filter(|&c| *c == ':').count();
        if number_of_colons != 1 {
            return Err(DataModelError::ParsingError(format!("too many colons in link value: {}", self.0)))
        }
        let position = chars.iter().position(|&c| c == ':').unwrap();

        let (ontology, resource) = self.0.split_at(position + 1);
        // if colon at beginning: current ontology
        if position == 0 {
            Ok(LinkElement{ ontology: onto_name, resource: resource.to_string()})
        }
        else {
            // else: other ontology
            Ok(LinkElement{ontology: ontology.to_string(), resource: resource.to_string()})
        }
    }
}
