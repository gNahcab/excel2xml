use std::fmt::Debug;
use hcl::{Block};
use crate::expression_trait::ExpressionTransform;
use crate::hcl_info::errors::HCLDataError;
use crate::hcl_info::methods_domain::create_loop::{Create, WrapperCreateLoop};
use crate::hcl_info::methods_domain::wrapper_trait::Wrapper;

pub struct WrapperCreateMethod(pub(crate) Block);
#[derive(Debug)]
struct TransientStructureInventMethod {
    output: String,
    prefix: Option<String>,
    suffix: Option<String>,
    create_loop: Option<Create>,
}

impl TransientStructureInventMethod {
    fn new(output: String) -> TransientStructureInventMethod {
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
        let mut transient_structure = TransientStructureInventMethod::new(self.0.get_output()?);
        
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "prefix" => {
                    transient_structure.add_prefix(attribute.expr.to_string_2()?)?;
                }
                "suffix" => {
                    transient_structure.add_suffix(attribute.expr.to_string_2()?)?;
                }
                _ => {
                    return Err(HCLDataError::ParsingError(format!("found this unknown attribute '{:?}' in method '{:?}'.", attribute, transient_structure.output)));
                }
            }
        }
        for block in self.0.blocks() {
            match block.identifier.replace("\\", "").as_str() {
                "integer" => {
                    let create_loop = WrapperCreateLoop(block.to_owned()).to_integer()?;
                    transient_structure.add_create_loop(create_loop)?
                }
                _ => {
                    return Err(HCLDataError::ParsingError(format!("found this unknown block-identifier '{:?}' in method with output'{:?}'.", block.identifier.as_str(), transient_structure.output)));
                }

            }
        }
        transient_structure.is_consistent()?;
        let invent_method = CreateMethod::new(transient_structure);
        Ok(invent_method)
    }
}
#[derive(Debug, Clone)]
pub struct CreateMethod {
    pub output: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub invent_loop: Create,
}


impl CreateMethod {
    fn new(transient_structure: TransientStructureInventMethod) -> CreateMethod {
        CreateMethod {
            output: transient_structure.output,
            prefix: transient_structure.prefix,
            suffix: transient_structure.suffix,
            invent_loop: transient_structure.create_loop.unwrap(),
        }
    }
}
