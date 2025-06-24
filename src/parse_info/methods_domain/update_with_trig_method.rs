use hcl::{Expression, Identifier};
use crate::expression_trait::ExpressionTransform;
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::header_value::{HeaderMethods, HeaderValue};
use crate::parse_info::methods_domain::wrapper_trait_block::Wrapper;

pub struct WrapperUpdateWithTrig(pub(crate) hcl::Block);

impl WrapperUpdateWithTrig {
    pub(crate) fn to_update_with_trig_method(&self) -> Result<UpdateWithTrigMethod, HCLDataError> {
        let mut transient_structure = TransientStructureUpdateWithTrigMethod::new(self.0.get_output()?);
        self.0.no_blocks()?;
        let attributes: Vec<&_> = self.0.attributes();
        if attributes.len() != 1 {
            return Err(HCLDataError::InputError(format!("Only one attribute allowed in update-with-trig-method, but found: '{}', attributes: '{:?}'", attributes.len(), attributes)));
        }
        let attribute = attributes.get(0).unwrap();
        match attribute.key.as_str() {
            "input" => {
                transient_structure.add_input(&attribute.expr)?;
            }
            _ => {
                return Err(HCLDataError::InputError(format!("Unknown key '{}' in update-with-trig-method, only 'input' allowed.", attribute.key.as_str())))
            }
        }
        Ok(UpdateWithTrigMethod::new(transient_structure))
    }
}

#[derive(Debug, Clone)]
pub struct UpdateWithTrigMethod {
    pub(crate) output: String,
    pub input: HeaderValue,
}

impl UpdateWithTrigMethod {
    fn new(transient_structure: TransientStructureUpdateWithTrigMethod) -> UpdateWithTrigMethod {
        UpdateWithTrigMethod{ output: transient_structure.output, input: transient_structure.input.unwrap()}
    }
}

#[derive(Debug)]
struct TransientStructureUpdateWithTrigMethod{
    output: String,
    input: Option<HeaderValue>,
}

impl TransientStructureUpdateWithTrigMethod {
    fn new(output: String) -> TransientStructureUpdateWithTrigMethod {
        TransientStructureUpdateWithTrigMethod {
            output,
            input: None,
        }
    }
    fn add_input(&mut self, input: &Expression) -> Result<(), HCLDataError> {
        let input_header_value = input.to_header_value()?;
        self.input = Option::from(input_header_value);
        Ok(())
    }
}
