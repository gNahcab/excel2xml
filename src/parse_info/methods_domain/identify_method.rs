use hcl::{Block, Expression};
use crate::hcl_info::errors::HCLDataError;
use crate::hcl_info::methods_domain::wrapper_trait::Wrapper;

pub struct WrapperIdentifyMethod (pub(crate) Block);

impl WrapperIdentifyMethod {
    pub(crate) fn to_identify_method(&self) -> Result<IdentifyMethod, HCLDataError> {
        let mut transient_structure = TransientStructureIdentifyMethod::new();

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
                "output" => {
                    match &attribute.expr {
                        Expression::String(output) => {
                            transient_structure.add_output(output.to_owned())?;
                        }
                        _ => {
                            return Err(HCLDataError::ParsingError(format!("identify-methods: output-attribute '{}' is not a string", attribute.key.as_str())));
                        }
                    }
                }
                "exchange" => {
                    match &attribute.expr {
                        Expression::Array(array) => {
                            let array = exchange_array(array)?;
                            transient_structure.add_exchange(array)?;
                        }
                        _ => {
                            return Err(HCLDataError::ParsingError(format!("identify-methods: exchange-attribute '{:?}' is not an array", attribute.expr)));
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

fn exchange_array(array: &Vec<Expression>) -> Result<Vec<String>, HCLDataError> {
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
    Ok(new_array)
}

#[derive(Debug, Clone)]
pub struct IdentifyMethod {
    pub resource_name: String,
    pub exchange: Vec<String>,
    pub output: String
}

impl IdentifyMethod {
    fn new(transient_structure_identify_method: TransientStructureIdentifyMethod) -> Self {
        IdentifyMethod {
            resource_name: transient_structure_identify_method.resource_name.unwrap(),
            exchange: transient_structure_identify_method.exchange.unwrap(),
            output: transient_structure_identify_method.output.unwrap(),
        }
    }
}

#[derive(Debug)]
struct TransientStructureIdentifyMethod {
    resource_name: Option<String>,
    exchange: Option<Vec<String>>,
    output: Option<String>
}

impl TransientStructureIdentifyMethod {
    pub(crate) fn add_resource(&mut self, resource_name: String) -> Result<(), HCLDataError> {
        if self.resource_name.is_some() {
            return Err(HCLDataError::ParsingError(format!("identify-method: Mutliple resource-attributes. First: {}, second: {}", &self.resource_name.as_ref().unwrap(), resource_name)))
        }
        self.resource_name = Some(resource_name);
        Ok(())
    }
    pub(crate) fn add_output(&mut self, output: String) -> Result<(), HCLDataError> {
        if self.output.is_some() {
            return Err(HCLDataError::ParsingError(format!("identify-method: Mutliple output-attributes. First: {}, second: {}", &self.output.as_ref().unwrap(), output)))
        }
        self.output = Some(output);
        Ok(())
    }
    pub(crate) fn add_exchange(&mut self, exchange: Vec<String>) -> Result<(), HCLDataError> {
        if self.exchange.is_some() {
            return Err(HCLDataError::ParsingError(format!("identify-method: Mutliple exchange-attributes. First: {:?}, second: {:?}", &self.exchange.as_ref().unwrap(), exchange)))
        }
        self.exchange = Some(exchange);
        Ok(())
    }
    fn is_correct(&self) -> Result<(), HCLDataError> {
        if self.resource_name.is_none() {
            return Err(HCLDataError::ParsingError(format!("identify-method: resource-attribute not found: {:?}", self)))
        }
        if self.exchange.is_none() {
            return Err(HCLDataError::ParsingError(format!("identify-method: exchange-attribute not found: {:?}", self)))
        }
        if self.output.is_none() {
            return Err(HCLDataError::ParsingError(format!("identify-method: output-attribute not found: {:?}", self)))
        }
        if self.output.as_ref().unwrap() != self.exchange.as_ref().unwrap().get(0).unwrap() {
            return Err(HCLDataError::ParsingError(format!("identify-method: output-value '{:?}' should be identical with the first value in exchange: {:?}", self.output.as_ref().unwrap(), self.exchange.as_ref().unwrap())))
        }
        Ok(())
    }
}

impl TransientStructureIdentifyMethod {
    fn new() -> TransientStructureIdentifyMethod {
        TransientStructureIdentifyMethod {
            resource_name: None,
            exchange: None,
            output: None,
        }
    }
}
