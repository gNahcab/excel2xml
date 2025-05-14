use std::fmt::Debug;
use hcl::{Block};
use crate::expression_trait::ExpressionTransform;
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::methods_domain::create_loop::{Create, WrapperCreateLoop};
use crate::parse_info::methods_domain::integer_create::{IntegerCreate, WrapperIntegerCreate};
use crate::parse_info::methods_domain::permissions_create::{PermissionsCreate, WrapperPermissionsCreate};
use crate::parse_info::methods_domain::wrapper_trait_block::Wrapper;

pub struct WrapperCreateMethod(pub(crate) Block);
#[derive(Debug)]
struct TransientStructureInventMethod {
    output: String,
    prefix: Option<String>,
    suffix: Option<String>,
    create_loop: Option<Create>,
}

impl TransientStructureInventMethod {
    fn new(output: String, identifier: String) -> TransientStructureInventMethod {
        TransientStructureInventMethod {
            output,
            prefix: None,
            suffix: None,
            create_loop: None,
        }
    }
    pub(crate) fn add_prefix(&mut self, prefix: String) -> Result<(), HCLDataError>{
        if self.prefix.is_some() {
            return Err(HCLDataError::ParsingError(format!("method: '{:?}' has multiple prefix-attributes", self)));
        }
        self.prefix = Option::from(prefix);
        Ok(())
    }
    pub(crate) fn add_suffix(&mut self, suffix: String) -> Result<(), HCLDataError>{
        if self.suffix.is_some() {
            return Err(HCLDataError::ParsingError(format!("method: '{:?}' has multiple suffix-attributes", self)));
        }
        self.suffix = Option::from(suffix);
        Ok(())
    }
    pub(crate) fn add_create_loop(&mut self, create_loop: Create) -> Result<(), HCLDataError>{
        if self.create_loop.is_some() {
            return Err(HCLDataError::ParsingError(format!("method: '{:?}' has multiple create_loop-attributes", self)));
        }
        self.create_loop = Option::from(create_loop);
        Ok(())

    }

    pub(crate) fn is_consistent(&self) -> Result<(), HCLDataError> {
        if self.create_loop.is_none() {
            return Err(HCLDataError::ParsingError(format!("invent-method: '{:?}' doesn't have an invent-loop provided", self)));
        }
        // suffix, prefix are optional
        Ok(())
    }
}


impl WrapperCreateMethod {

    pub(crate) fn to_create_method(&self) -> Result<CreateMethod, HCLDataError> {
        let (method_type, output) = self.0.get_output_two()?;
        if method_type.is_empty() {
            return Err(HCLDataError::InputError(format!("Create-Method: Method-type empty in '{:?}'", self.0.body)));
        }
        if output.is_empty() {
            return Err(HCLDataError::InputError(format!("Create-Method: output empty in '{:?}'", self.0.body)));
        }
        match method_type.as_str() {
            "integer" => {
                let integer_create_method = WrapperIntegerCreate(self.0.to_owned()).to_integer_create_method(output)?;
                Ok(CreateMethod::IntegerCreateMethod(integer_create_method))
            }
            "permissions" | "permission" => {
                let permissions_create_method = WrapperPermissionsCreate(self.0.to_owned()).to_permissions_create_method(output)?;
                Ok(CreateMethod::PermissionsCreateMethod(permissions_create_method))
            }
            _ => {
                Err(HCLDataError::ParsingError(format!("found this unknown method-type '{:?}' in method with output'{:?}'.", method_type, output)))
            }

        }
    }
}
#[derive(Debug, Clone)]
pub enum CreateMethod {
    IntegerCreateMethod(IntegerCreate),
    PermissionsCreateMethod(PermissionsCreate),
}
#[derive(Debug, Clone)]
pub struct old_CreateMethod {
    pub output: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub invent_loop: Create,
}


impl old_CreateMethod {
    fn new(transient_structure: TransientStructureInventMethod) -> old_CreateMethod {
        old_CreateMethod {
            output: transient_structure.output,
            prefix: transient_structure.prefix,
            suffix: transient_structure.suffix,
            invent_loop: transient_structure.create_loop.unwrap(),
        }
    }
}
