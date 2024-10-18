use std::str::ParseBoolError;
use rust_decimal::{Decimal, Error};
use regex::{Captures, Regex};
use crate::json2datamodel::domain::dasch_list::{DaSCHList, ListNode};
use crate::json2datamodel::domain::data_model::DataModel;
use crate::json2datamodel::domain::object::ValueObject;
use crate::json2datamodel::domain::property::Property;
use crate::parse_xlsx::errors::ExcelDataError;

pub struct ValueField {
    values: Vec<String>
}

impl ValueField {
    fn new(values: Vec<String>) -> Self {
        ValueField{ values }
    }
}

pub struct ValueFieldWrapper(pub(crate) Vec<String>);
impl ValueFieldWrapper {
    pub(crate) fn to_value_field(&self, data_model: &DataModel, header: &String) -> Result<ValueField, ExcelDataError> {
        let curr_prop = match data_model.properties.iter().find(|property|property.name.eq(&header.to_owned())) {
            None => {
                // should never happen
                return Err(ExcelDataError::InputError(format!("cannot find header '{}' in datamodel-properties: '{:?}'", header, data_model.properties)))
            }
            Some(curr_prop) => {curr_prop}
        };
        match curr_prop.object {
            ValueObject::ListValue => {
                let list: &DaSCHList = match
                    data_model.lists.get(curr_prop.h_list.as_ref().unwrap()){
                    None => {
                        return Err(ExcelDataError::InputError(format!("cannot find hlist '{}' of property '{}' in lists of datamodel", &curr_prop.h_list.as_ref().unwrap(), curr_prop.name)));
                    }
                    Some(list) => {list}
                };
                correct_list_values(&self.0, list)?;
            }
            ValueObject::TextValue => {
                // we don't check if Text is correct
            }
            ValueObject::DateValue => {
                // check if date is valid DSP-Date
            }
            ValueObject::UriValue => {
                // we don't check if URI is correct
            }
            ValueObject::GeonameValue => {
                // we don't check if Geoname is correct
            }
            ValueObject::DecimalValue => {
                // check if parsing is possible
                for value in self.0.iter() {
                    let _ = match value.parse::<rust_decimal::Decimal>() {
                        Ok(decimal) => {decimal}
                        Err(error) => {
                            return Err(ExcelDataError::InputError(format!("cannot parse '{}' to decimal: {:?}",value, error)));
                        }
                    };
                }
            }
            ValueObject::ColorValue => {
                // we don't check if ColorValue is correct
            }
            ValueObject::IntValue => {
                // check if parsing is possible
                for value in self.0.iter() {
                    let _ = match value.parse::<usize>() {
                        Ok(integer) => { integer }
                        Err(error) => {
                            return Err(ExcelDataError::InputError(format!("cannot parse '{}' to integer: {:?}", value, error)));
                        }
                    };
                }
            }
            ValueObject::BooleanValue => {
                if self.0.len() > 1 {
                    return Err(ExcelDataError::ParsingError(format!("Boolean-values are only allowed single, but found multiple: '{:?}' for property '{}'", self.0, curr_prop.name)))
                }
                let _: bool = match self.0.get(0).unwrap().trim().parse::<bool>()
                {
                    Ok(bool_val) => {bool_val}
                    Err(error) => {
                        return Err(ExcelDataError::ParsingError(format!("cannot parse '{}' from string to bool. Error message: {}", self.0.get(0).unwrap(),error)));
                    }
                };
            }
            ValueObject::TimeValue => {
                // yyyy-mm-ddThh:mm:ss.sssssssssssszzzzz
                // 2021-11-30T12:00:00+00:00
                let re = Regex::new(r"^\d{4}-\d{2}-\d{2}\D{1}\d{2}:\d{2}:\d{2}+\d{2}:\d{2}").unwrap();
                for value in self.0.iter() {
                    let _ = match  re.captures(value)  {
                        None => {
                            return Err(ExcelDataError::ParsingError(format!("cannot parse '{}' to TimeValue", value)));
                        }
                        Some(_) => {}
                    };
                }
            }
            ValueObject::Representation => {
                // we don't check if representation exists
            }
            ValueObject::ResLinkValue(_) => {
                // we don't check if reslinkvalue exists
            }
        }
        Ok(ValueField::new(self.0.iter().map(|value|value.to_owned()).collect::<Vec<String>>()))
    }
}

fn correct_list_values(values: &Vec<String>, list: &DaSCHList) -> Result<(), ExcelDataError> {
    // check if one name of node is equal to value
    let node_names: Vec<String> = collect_node_names(&list.nodes);
    for value in values.iter() {
        if value.is_empty() {
            // we just ignore for now if value is empty
            continue;
        }
        match node_names.contains(&value) {
            true => {}
            false => {
               return Err(ExcelDataError::InputError(format!("cannot find '{}' in list '{}'", value, list.name)));
            }
        }
    };
    Ok(())
}

fn collect_node_names(nodes: &Vec<ListNode>) -> Vec<String> {
    let mut nodes: Vec<ListNode> = nodes.iter().map(|node|node.to_owned()).collect();
    let mut names = vec![];
    while !nodes.is_empty() {
        let node = nodes.pop().unwrap();
        names.push(node.name);
        node.nodes.iter().for_each(|node|nodes.push(node.to_owned()));
    }
    names
}
