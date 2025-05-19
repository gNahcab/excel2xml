use hcl::{Block, Expression};
use crate::expression_trait::ExpressionTransform;
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::header_value::{HeaderMethods, HeaderValue};
use crate::parse_info::methods_domain::wrapper_trait_block::Wrapper;

#[derive(Debug, Clone)]
pub struct AlterMethod{
    pub(crate) prefix: Option<String>,
    pub(crate) suffix: Option<String>,
    pub(crate) input: HeaderValue,
    pub(crate) output: String
}

struct TransientStructureAlterMethod {
    prefix: Option<String>,
    suffix: Option<String>,
    input: Option<HeaderValue>,
    output: String
}

impl TransientStructureAlterMethod {
    fn new(output: String) -> Self {
        Self{
            prefix: None,
            suffix: None,
            input: None,
            output,
        }
    }
    pub(crate) fn add_input(&mut self, input: HeaderValue) -> Result<(), HCLDataError> {
        if self.input.is_some() {
            return Err(HCLDataError::ParsingError(format!("Alter-method: found multiple inputes: First: {:?}, Second: {:?}", self.input.as_ref().unwrap(), input)));
        }
        self.input = Some(input);
        Ok(())
    }
    pub(crate) fn add_suffix(&mut self, suffix: String) -> Result<(), HCLDataError> {
        if self.suffix.is_some() {
            return Err(HCLDataError::ParsingError(format!("Alter-method: found multiple suffixes: First: {}, Second: {}", self.suffix.as_ref().unwrap(), suffix)));
        }
        self.suffix = Some(suffix);
        Ok(())
    }
    pub(crate) fn add_prefix(&mut self, prefix: String) -> Result<(), HCLDataError> {
        if self.prefix.is_some() {
            return Err(HCLDataError::ParsingError(format!("Alter-method: found multiple prefixes: First: {}, Second: {}", self.prefix.as_ref().unwrap(), prefix)));
        }
        self.prefix = Some(prefix);
        Ok(())
    }
    pub(crate) fn is_consistent(&self) -> Result<(), HCLDataError> {
        if self.input.is_none() {
            return Err(HCLDataError::ParsingError(format!("Alter-method: not found 'input' value in alter-method with output: {}", self.output)));
        }
        // if suffix and prefix is None, the result doesn't change, return error
        if self.suffix.is_none() && self.prefix.is_none() {
            return Err(HCLDataError::ParsingError(format!("Alter-method: Found no suffix and no prefix. Input and output remain the same. Input: {:?}, Output:{}", self.input.as_ref().unwrap(), self.output)))
        }
        Ok(())
    }
}

impl AlterMethod {
    fn new(transient_structure_alter_method: TransientStructureAlterMethod) -> Self {
        Self {
            prefix:transient_structure_alter_method.prefix,
            suffix: transient_structure_alter_method.suffix,
            input: transient_structure_alter_method.input.unwrap(),
            output: transient_structure_alter_method.output
        }
    }
}

pub struct WrapperAlterMethod(pub(crate) Block);

impl WrapperAlterMethod {
    pub(crate) fn to_alter_method(&self) -> Result<AlterMethod, HCLDataError> {
        let mut transient_structure = TransientStructureAlterMethod::new(self.0.get_output()?);
        self.0.no_blocks()?;
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "input" => {
                    let header_value = attribute.expr.to_owned().to_header_value()?;
                    transient_structure.add_input(header_value)?;
                }
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
        transient_structure.is_consistent()?;
        let combine_method = AlterMethod::new(transient_structure);
        Ok(combine_method)
    }
}

