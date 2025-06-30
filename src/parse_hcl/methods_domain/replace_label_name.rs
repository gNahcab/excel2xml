use hcl::Expression;
use crate::expression_trait::ExpressionTransform;
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::header_value::{HeaderMethods, HeaderValue};
use crate::parse_hcl::methods_domain::wrapper_trait_block::Wrapper;

pub struct WrapperReplaceLabelNameMethod(pub(crate) hcl::Block);

#[derive(Debug)]
struct TransientStructureReplaceLabelNameMethod {
    output: String,
    input: Option<HeaderValue>,
    list_name: Option<String>,
}

impl TransientStructureReplaceLabelNameMethod {
    fn new(output: String) -> TransientStructureReplaceLabelNameMethod {
        TransientStructureReplaceLabelNameMethod {
            output,
            input: None,
            list_name: None,
        }
    }
    fn add_input(&mut self, expression: Expression) -> Result<(), HCLDataError> {
        if self.input.is_some() {
            return Err(HCLDataError::ParsingError(format!("found multiple input-attributes  in method '{:?}'.", self.output)));
        }
        let header_value = expression.to_header_value()?;
        self.input = Option::from(header_value);
        Ok(())
    }
    fn add_list_name(&mut self, expression: Expression) -> Result<(), HCLDataError> {
        if self.list_name.is_some() {
            return Err(HCLDataError::ParsingError(format!("found multiple list-name-attributes  in method '{:?}'.", self.output)));
        }
        self.list_name = Option::from(expression.to_string_2()?);
        Ok(())
    }
    fn is_consistent(&self) -> Result<(), HCLDataError> {
        if self.input.is_none() {
            return Err(HCLDataError::ParsingError(format!("replace_label_name-method '{:?}' doesn't have an input-attribute provided", self)));
        }
        if self.list_name.is_none() {
            return Err(HCLDataError::ParsingError(format!("replace_label_name-method '{:?}' doesn't have a list-name-attribute provided", self)));
        }
        Ok(())
    }
}
impl WrapperReplaceLabelNameMethod {
    pub(crate) fn to_replace_label_name_method(&self) -> Result<ReplaceLabelNameMethod, HCLDataError> {
        let mut transient_structure = TransientStructureReplaceLabelNameMethod::new(self.0.get_output()?);
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "input" => {
                    transient_structure.add_input(attribute.expr.to_owned())?;
                }
                "list_name" => {
                    transient_structure.add_list_name(attribute.expr.to_owned())?;
                }
                _ => {
                    return Err(HCLDataError::ParsingError(format!("found this unknown attribute '{:?}' in method '{:?}'.", attribute, transient_structure.output)));
                }
            }
        }
        transient_structure.is_consistent()?;

        let replace_method = ReplaceLabelNameMethod::new(transient_structure)?;
        Ok(replace_method)
    }
}
#[derive(Debug, Clone)]
pub struct ReplaceLabelNameMethod {
    pub output: String,
    pub input: HeaderValue,
    pub list_name: String,
}


impl ReplaceLabelNameMethod {
    fn new(transient_structure: TransientStructureReplaceLabelNameMethod) -> Result<ReplaceLabelNameMethod, HCLDataError> {
        Ok(ReplaceLabelNameMethod {
            output: transient_structure.output,
            input: transient_structure.input.unwrap(),
            list_name: transient_structure.list_name.unwrap(),
        })
    }
    pub(crate) fn is_correct(&self) -> Result<(), HCLDataError> {
        if self.input.is_equal(&self.output) {
            return Err(HCLDataError::ParsingError(format!("method has the same in- and output-String, which is forbidden: '{:?}'", self.input)));
        }
        Ok(())
    }
}
