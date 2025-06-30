use hcl::{Block, BlockLabel, Expression};
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::methods_domain::wrapper_trait_block::Wrapper;

pub struct WrapperIdentifyMethod (pub(crate) Block);

impl WrapperIdentifyMethod {
    pub(crate) fn to_identify_method(&self) -> Result<IdentifyMethod, HCLDataError> {
        let output = output(&self.0.labels)?;
        let mut transient_structure = TransientStructureIdentifyMethod::new(output);

        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "resource" => {
                    match &attribute.expr {
                        Expression::String(resource) => {
                            transient_structure.add_resource(resource.to_owned())?;
                        }
                        _ => {
                            return Err(HCLDataError::ParsingError(format!("identify-methods: resource-attribute '{}' is not a string", attribute.key.as_str())));
                        }
                    }
                }
                "exchange" => {
                    match &attribute.expr {
                        Expression::Array(array) => {
                            let (key, value) = key_value(array)?;
                            transient_structure.add_exchange(key, value)?;
                        }
                        _ => {
                            return Err(HCLDataError::ParsingError(format!("identify-methods: exchange-attribute '{:?}' is not an array", attribute.expr)));
                        }
                    }
                }
                "input" => {
                    match &attribute.expr {
                        Expression::String(input_header) => {
                            transient_structure.add_input(input_header.to_owned())?;
                        }
                        _=> {
                            return Err(HCLDataError::ParsingError(format!("identify-methods: input-attribute '{:?}' is not a string", attribute.expr)));
                        }
                    }
                }
                _ => {
                    return Err(HCLDataError::ParsingError(format!("found this unknown attribute '{:?}' in identify-method.", attribute)));
                }
            }
        }
        transient_structure.is_correct()?;
        let identify_method = IdentifyMethod::new(transient_structure);
        Ok(identify_method)
    }
}

fn output(labels: &Vec<BlockLabel>) -> Result<String, HCLDataError> {
    if labels.len() != 1 {
        return Err(HCLDataError::ParsingError(format!("One label for 'identify-method' allowed, but found more or less: '{:?}'", labels)));
    }
    let label = match labels.get(0).unwrap() {
        BlockLabel::String(label) => {label.to_owned()}
        BlockLabel::Identifier(_id) => {
            return Err(HCLDataError::ParsingError(format!("Cannot parse label of 'identify-method' to string, because it is an identifier: '{:?}'", _id)));
        }
    };
    Ok(label)
}

fn key_value(array: &Vec<Expression>) -> Result<(String, String), HCLDataError> {
    let mut new_array = vec![];
    for expr in array.iter() {
        match expr {
            Expression::String(header) => {
                new_array.push(header.to_owned());
            }
            _ => {
                return Err(HCLDataError::ParsingError(format!("identify-methods: resource-attribute must be a string: '{:?}'", expr)))
            }
        }
    }
    if new_array.len() != 2 {
        return Err(HCLDataError::ParsingError(format!("identify-methods: array should contain two values: '{:?}'", new_array)))
    }
    Ok((new_array[0].to_owned(), new_array[1].to_owned()))
}

#[derive(Debug, Clone)]
pub struct IdentifyMethod {
    pub resource_name: String,
    pub key: String,
    pub value: String,
    pub output: String,
    pub input: String,
}

impl IdentifyMethod {
    fn new(transient_structure_identify_method: TransientStructureIdentifyMethod) -> Self {
        IdentifyMethod {
            resource_name: transient_structure_identify_method.resource_name.unwrap(),
            key: transient_structure_identify_method.key.unwrap(),
            value: transient_structure_identify_method.value.unwrap(),
            input: transient_structure_identify_method.input.unwrap(),
            output: transient_structure_identify_method.output,
        }
    }
}

#[derive(Debug)]
struct TransientStructureIdentifyMethod {
    resource_name: Option<String>,
    key: Option<String>,
    value: Option<String>,
    input: Option<String>,
    output: String,
}


impl TransientStructureIdentifyMethod {
    pub(crate) fn add_resource(&mut self, resource_name: String) -> Result<(), HCLDataError> {
        if self.resource_name.is_some() {
            return Err(HCLDataError::ParsingError(format!("identify-method: Mutliple resource-attributes. First: {}, second: {}", &self.resource_name.as_ref().unwrap(), resource_name)))
        }
        self.resource_name = Some(resource_name);
        Ok(())
    }
    pub(crate) fn add_exchange(&mut self, key: String, value: String) -> Result<(), HCLDataError> {
        if self.key.is_some() || self.value.is_some() {
            return Err(HCLDataError::ParsingError(format!("identify-method: Multiple exchange-attributes: first: {}:{} ; second: {}:{}.", &self.key.as_ref().unwrap(), &self.value.as_ref().unwrap(), key, value)))
        }
        self.key = Some(key);
        self.value = Some(value);
        Ok(())
    }
    pub(crate) fn add_input(&mut self, input: String) -> Result<(), HCLDataError> {
        if self.input.is_some() {
            return Err(HCLDataError::ParsingError(format!("identify-method: Multiple input-attributes. First: {}, second: {}", &self.input.as_ref().unwrap(), input)));
        }
        self.input = Some(input);
        Ok(())
    }
    fn is_correct(&self) -> Result<(), HCLDataError> {
        if self.resource_name.is_none() {
            return Err(HCLDataError::ParsingError(format!("identify-method: resource-attribute not found: {:?}", self)))
        }
        if self.input.is_none() {
            return Err(HCLDataError::ParsingError(format!("identify-method: input-attribute not found: {:?}", self)))
        }
        if self.key.is_none() || self.value.is_none() {
            return Err(HCLDataError::ParsingError(format!("identify-method: exchange-attribute not found: {:?}", self)))
        }
        Ok(())
    }
    fn new(output: String) -> TransientStructureIdentifyMethod {
        TransientStructureIdentifyMethod {
            resource_name: None,
            key: None,
            value: None,
            input: None,
            output
        }
    }
}

