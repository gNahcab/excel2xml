use std::collections::HashMap;
use std::num::ParseIntError;
use std::ops::Index;
use regex::Regex;
use crate::parse_dm::domain::dasch_list::{DaSCHList, ListNode};
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::gui_element::GUIElement;
use crate::parse_dm::domain::object::ValueObject;
use crate::parse_dm::domain::property::Property;
use crate::parse_hcl::domain::prop_supplement::{PropSupplType, PropSupplement};
use crate::parse_xlsx::domain::subheader_value::{SubheaderValues};
use crate::parse_xlsx::domain::dasch_value::{DaschValue, TransientDaschValue, WrapperDaschValue};
use crate::parse_xlsx::domain::encoding::{Encoding, EncodingWrapper};
use crate::parse_xlsx::domain::permissions::{Permissions, PermissionsWrapper};
use crate::parse_xlsx::errors::ExcelDataError;

#[derive(Clone, Debug)]
pub struct DaschValueField {
    pub values: Vec<DaschValue>,
    pub propname: String
}

impl DaschValueField {
    fn new(values: Vec<DaschValue>, propname: String) -> Self {
        Self{ values, propname }

    }
}
pub struct TransientSupplementValueField {
    pub(crate) comment: Option<Vec<String>>,
    pub(crate) encoding: Option<Vec<Encoding>>,
    pub(crate) permissions: Option<Vec<Permissions>>,
}

impl TransientSupplementValueField {
    fn new() -> Self {
        Self {
            comment: None,
            encoding: None,
            permissions: None,
        }
    }

    fn same_length(&self) -> Result<(), ExcelDataError> {
        let mut length = None;
        if self.encoding.is_some() {
            length = Some(self.encoding.as_ref().unwrap().len());
        }
        if self.comment.is_some() {
            let curr_length = self.comment.as_ref().unwrap().len();
            if length.is_none() {
                length = Some(curr_length);
            } else {
                if curr_length != length.unwrap() {
                    return Err(ExcelDataError::InputError(format!("Found differing length of supplement-values. Encoding: '{:?}', Comment: '{:?}'", &self.encoding, &self.comment)));
                }
            }
        }
        if self.permissions.is_some() {
            if length.is_some() {
                let curr_length = self.permissions.as_ref().unwrap().len();
                if curr_length != length.unwrap() {
                    return Err(ExcelDataError::InputError(format!("Found differing length of supplement-values. Comment: '{:?}', Permissions: '{:?}'", &self.comment, &self.permissions)));
                }
            }
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.comment.is_none() && self.permissions.is_none() && self.encoding.is_none()
    }
    fn add_prop_suppl(&mut self, prop_suppl: &PropSupplement, values: &Vec<String>) -> Result<(), ExcelDataError> {
        match prop_suppl.suppl_type {
            PropSupplType::Comment => {
                self.add_comment(values.to_vec())?;
            }
            PropSupplType::Encoding => {
                let mut encodings = vec![];
                for value in values.iter() {
                    let encoding = EncodingWrapper(value.to_string()).to_encoding()?;
                    encodings.push(encoding);
                }
                self.add_encoding(encodings)?;
            }
            PropSupplType::Permissions => {
                let mut permissions = vec![];
                for value in values.iter() {
                    let permission = PermissionsWrapper(value.to_string()).to_permissions()?;
                    permissions.push(permission);
                }
                self.add_permissions(permissions)?;
            }
        }
        Ok(())
    }

    fn add_comment(&mut self, comment: Vec<String>) -> Result<(), ExcelDataError> {
        if self.comment.is_some() {
            return Err(ExcelDataError::InputError(format!("Duplicate comment in dasch-value-field. First:  '{:?}', second: '{:?}'", self.comment.as_ref().unwrap(), comment)));
        }
        self.comment = Some(comment);
        Ok(())
    }
    fn add_encoding(&mut self, encoding: Vec<Encoding>) -> Result<(), ExcelDataError> {
        if self.encoding.is_some() {
            return Err(ExcelDataError::InputError(format!("Duplicate encoding in dasch-value-field. First:  '{:?}', second: '{:?}'", self.encoding.as_ref().unwrap(), encoding)));
        }
        self.encoding = Some(encoding);
        Ok(())
    }
    fn add_permissions(&mut self, permissions: Vec<Permissions>) -> Result<(), ExcelDataError> {
        if self.permissions.is_some() {
            return Err(ExcelDataError::InputError(format!("Duplicate permissions in dasch-value-field. First:  '{:?}', second: '{:?}'", self.permissions.as_ref().unwrap(), permissions)));
        }
        self.permissions = Some(permissions);
        Ok(())
    }
}


pub struct FieldsWrapper(pub HashMap<String, Vec<String>>, pub HashMap<String, Vec<(PropSupplement, Vec<String>)>>);

struct DaschValueFieldWrapper( Vec<String>);


impl DaschValueFieldWrapper {
    fn to_dasch_value_field(&self, prop_name: &String, maybe_suppl_value: Option<&TransientSupplementValueField>, data_model: &DataModel, set_permissions: bool) -> Result<DaschValueField, ExcelDataError> {
        let curr_prop = match data_model.properties.iter().find(|property| property.name.eq(&prop_name.to_owned())) {
            None => {
                // should never happen
                return Err(ExcelDataError::InputError(format!("cannot find header '{}' in datamodel-properties: '{:?}'", prop_name, data_model.properties)));
            }
            Some(curr_prop) => { curr_prop }
        };
        self.check_values(curr_prop, data_model)?;
        let mut dasch_values = vec![];
        for (pos, value) in self.0.iter().enumerate() {
            let dasch_value = WrapperDaschValue(value.to_owned()).to_dasch_value(pos, maybe_suppl_value, &curr_prop.gui_element, set_permissions)?;
            dasch_values.push(dasch_value);
        }
        Ok(DaschValueField::new(dasch_values, prop_name.to_owned()))
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
                match curr_prop.gui_element {
                    GUIElement::RICHTEXT => {
                        // everything allowed?
                    }
                    GUIElement::SIMPLETEXT => {
                        // no newline allowed if SimpleText
                        for value in self.0.iter() {
                            if value.contains("\n") {
                                return Err(ExcelDataError::ParsingError(format!("The following value '{}' of property '{:?}' contains newline but newline is forbidden in SimpleText.", value, curr_prop)))
                            }
                        }
                        // no xml-tags allowed if SimpleText or TextArea
                    }
                    GUIElement::TEXTAREA => {
                        // no xml-tags allowed if SimpleText or TextArea
                    }
                    _ => {
                        return Err(ExcelDataError::ParsingError(format!("Only 'RichtText, 'SimpleText' or 'TextArea' allowed for TextValue, but found: {:?}", curr_prop.gui_element)));
                    }
                }
            }
            ValueObject::DateValue => {
                // check if date is valid DSP-Date
            }
            ValueObject::UriValue => {
                // we don't check if URI is correct
            }
            ValueObject::GeonameValue => {
                // we don't check if Geoname is correct, but we check if Geoname is a number
                for value in self.0.iter() {
                    let _ = match value.parse::<usize>() {
                        Ok(_) => {}
                        Err(_) => {
                            return Err(ExcelDataError::InputError(format!("Cannot parse Geoname-Number of '{:?}' to usize: {}", curr_prop, value)));
                        }
                    };
                }
            }
            ValueObject::DecimalValue => {
                // check if parsing is possible
                for value in self.0.iter() {
                    let _ = match value.parse::<rust_decimal::Decimal>() {
                        Ok(decimal) => { decimal }
                        Err(error) => {
                            return Err(ExcelDataError::InputError(format!("cannot parse '{}' to decimal: {:?} in {:?}", value, error, curr_prop)));
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
                            return Err(ExcelDataError::InputError(format!("cannot parse '{}' to integer: {:?} in {:?}", value, error, curr_prop)));
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
                // yyyy-mm-ddThh:mm:ss
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
}
impl FieldsWrapper {

    pub(crate) fn to_dasch_value_fields(&self, data_model: &DataModel, set_permissions: bool) -> Result<Vec<DaschValueField>, ExcelDataError> {
        let mut dasch_value_fields: Vec<DaschValueField> = vec![];
        let mut prop_name_to_transient_suppl_value = HashMap::new();
        for (prop_name, prop_suppl_values) in self.1.iter() {
            let mut transient_dasch_value_field = TransientSupplementValueField::new();
            for (prop_suppl, values) in prop_suppl_values.iter() {
                transient_dasch_value_field.add_prop_suppl(prop_suppl, values)?;
            }
            if !transient_dasch_value_field.is_empty() {
                transient_dasch_value_field.same_length()?;
                prop_name_to_transient_suppl_value.insert(prop_name, transient_dasch_value_field);
            }
        }
        for (prop_name, values) in self.0.iter() {
            let maybe_suppl_value =  prop_name_to_transient_suppl_value.get(prop_name);
            let dasch_value_field = DaschValueFieldWrapper(values.to_vec()).to_dasch_value_field(prop_name, maybe_suppl_value, data_model, set_permissions)?;
            dasch_value_fields.push(dasch_value_field);
        }
        Ok(dasch_value_fields)
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
                    GUIElement::TEXTAREA => {
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
