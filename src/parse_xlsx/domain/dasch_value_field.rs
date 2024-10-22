use std::ops::Index;
use regex::Regex;
use crate::parse_dm::domain::dasch_list::{DaSCHList, ListNode};
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::gui_element::GUIElement;
use crate::parse_dm::domain::object::ValueObject;
use crate::parse_dm::domain::property::Property;
use crate::parse_xlsx::domain::subheader_value::{SubheaderValues};
use crate::parse_xlsx::domain::dasch_value::DaschValue;
use crate::parse_xlsx::domain::encoding::Encoding;
use crate::parse_xlsx::domain::permissions::Permissions;
use crate::parse_xlsx::errors::ExcelDataError;

pub struct DaschValueField {
    pub values: Vec<DaschValue>
}

impl DaschValueField {
    fn new(values: Vec<DaschValue>) -> Self {
        DaschValueField { values }
    }
}

pub struct ValueFieldWrapper(pub(crate) Vec<String>);
impl ValueFieldWrapper {
    pub(crate) fn to_dasch_value_field(&self, data_model: &DataModel, header: &String, subheader: Option<SubheaderValues>) -> Result<DaschValueField, ExcelDataError> {
        let curr_prop = match data_model.properties.iter().find(|property| property.name.eq(&header.to_owned())) {
            None => {
                // should never happen
                return Err(ExcelDataError::InputError(format!("cannot find header '{}' in datamodel-properties: '{:?}'", header, data_model.properties)));
            }
            Some(curr_prop) => { curr_prop }
        };
        self.check_values(curr_prop, data_model)?;
        let subheader = match subheader {
            None => {
                self.default_subheader(curr_prop)
            }
            Some(subheader) => {
                self.check_subheader(&subheader, curr_prop);
                subheader
            }
        };
        let dasch_values = self.dasch_values(subheader)?;
        Ok(DaschValueField::new(dasch_values))
    }
    fn dasch_values(&self, subheader: SubheaderValues) -> Result<Vec<DaschValue>, ExcelDataError>{
        self.sub_values_length_match(&subheader)?;
        let mut dasch_values = vec![];
        for (pos, value) in self.0.iter().enumerate() {
            let permissions = subheader.permissions.index(pos);
            let mut dasch_value = DaschValue::new(value.to_owned(), permissions.to_owned());
            if subheader.encodings.is_some() {
                let encoding = subheader.encodings.as_ref().unwrap().index(pos);
                dasch_value.add_encoding(encoding.to_owned());
            }
            if subheader.comments.is_some() {
                let comment = subheader.comments.as_ref().unwrap().index(pos);
                if !comment.is_empty() {
                    dasch_value.add_comment(comment.to_owned());
                }
            }
            dasch_values.push(dasch_value);
        }
        Ok(dasch_values)
    }
    fn check_values(&self, curr_prop: &Property, data_model: &DataModel) -> Result<(), ExcelDataError> {
        match curr_prop.object {
            ValueObject::ListValue => {
                let list: &DaSCHList = match
                data_model.lists.get(curr_prop.h_list.as_ref().unwrap()) {
                    None => {
                        return Err(ExcelDataError::InputError(format!("cannot find hlist '{}' of property '{}' in lists of datamodel", &curr_prop.h_list.as_ref().unwrap(), curr_prop.name)));
                    }
                    Some(list) => { list }
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
                        Ok(decimal) => { decimal }
                        Err(error) => {
                            return Err(ExcelDataError::InputError(format!("cannot parse '{}' to decimal: {:?}", value, error)));
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
                    Ok(bool_val) => { bool_val }
                    Err(error) => {
                        return Err(ExcelDataError::ParsingError(format!("cannot parse '{}' from string to bool. Error message: {}", self.0.get(0).unwrap(), error)));
                    }
                };
            }
            ValueObject::TimeValue => {
                // yyyy-mm-ddThh:mm:ss.sssssssssssszzzzz
                // 2021-11-30T12:00:00+00:00
                let re = Regex::new(r"^\d{4}-\d{2}-\d{2}\D{1}\d{2}:\d{2}:\d{2}+\d{2}:\d{2}").unwrap();
                for value in self.0.iter() {
                    let _ = match re.captures(value) {
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
        Ok(())
    }

    fn default_subheader(&self, curr_prop: &Property) -> SubheaderValues {
        let encodings = match curr_prop.object {
            ValueObject::TextValue => {
                match curr_prop.gui_element {
                    GUIElement::RICHTEXT => {
                        Some(self.0.iter().map(|_|Encoding::XML).collect())
                    }
                    GUIElement::SIMPLETEXT => {
                        Some(self.0.iter().map(|_|Encoding::UTF8).collect())
                    }
                    _ => {
                        panic!("shouldn't happen: Found GUIElement for TextValue I did not handle: {:?}", curr_prop.gui_element)
                    }
                }
            }
            _ => {
                // no encoding needed
                None
            }
        };
        let permissions = self.0.iter().map(|_|Permissions::DEFAULT).collect();
        SubheaderValues {
            permissions,
            encodings,
            comments: None,
        }
    }

    fn check_subheader(&self, subheader: &SubheaderValues, curr_prop: &Property) -> Result<(), ExcelDataError> {
        // the number of permissions, encodings, comments (if the latter two exist)
        // should match the number of values
        todo!()
    }

    fn sub_values_length_match(&self, subheader_value: &SubheaderValues) -> Result<(), ExcelDataError> {
        if self.0.len() != subheader_value.permissions.len() {
            return Err(ExcelDataError::ParsingError(format!("Permissions and values have different length! Values: '{:?}', Permissions: '{:?}'", &self.0, subheader_value.permissions)))

        }
        if subheader_value.encodings.is_some() {
            if self.0.len() != subheader_value.encodings.as_ref().unwrap().len() {
                return Err(ExcelDataError::ParsingError(format!("Encodings and values have different length! Values: '{:?}', Encodings: '{:?}'", &self.0, subheader_value.encodings.as_ref().unwrap())))

            }
        }
        if subheader_value.comments.is_some() {
            if self.0.len() != subheader_value.comments.as_ref().unwrap().len() {
                return Err(ExcelDataError::ParsingError(format!("Comments and values have different length! Values: '{:?}', Comments: '{:?}'", &self.0, subheader_value.comments.as_ref().unwrap())))
            }
        }
        Ok(())
    }
}

fn default_subheader() {
    todo!()
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
