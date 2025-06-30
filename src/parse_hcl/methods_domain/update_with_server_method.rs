use hcl::{Expression, Identifier};
use crate::expression_trait::ExpressionTransform;
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::header_value::{HeaderMethods, HeaderValue};
use crate::parse_hcl::methods_domain::wrapper_trait_block::Wrapper;

pub struct WrapperUpdateWithServer(pub(crate) hcl::Block);

impl WrapperUpdateWithServer {
    pub(crate) fn to_update_with_server_method(&self) -> Result<ReplaceWithIRI, HCLDataError> {
        let mut transient_structure = TransientStructureUpdateWithServerMethod::new(self.0.get_output()?);
        self.0.no_blocks()?;
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "input" => {
                    transient_structure.add_input(&attribute.expr)?;
                }
                "resource" => {
                    transient_structure.add_resource(attribute.expr.to_string_2()?);
                }
                _ => {
                    return Err(HCLDataError::InputError(format!("Unknown key '{}' in update-with-server-method, only 'input' allowed.", attribute.key.as_str())))
                }
            }
        }
        transient_structure.is_complete()?;
        Ok(ReplaceWithIRI::new(transient_structure))
    }
}

#[derive(Debug, Clone)]
pub struct ReplaceWithIRI {
    pub(crate) output: String,
    pub input: HeaderValue,
    pub resource: String
}

impl ReplaceWithIRI {
    fn new(transient_structure: TransientStructureUpdateWithServerMethod) -> ReplaceWithIRI {
        ReplaceWithIRI { output: transient_structure.output,
            input: transient_structure.input.unwrap(),
            resource: transient_structure.resource.unwrap()}
    }
}

#[derive(Debug)]
struct TransientStructureUpdateWithServerMethod {
    output: String,
    input: Option<HeaderValue>,
    resource: Option<String>
}

impl TransientStructureUpdateWithServerMethod {
    fn new(output: String) -> TransientStructureUpdateWithServerMethod {
        TransientStructureUpdateWithServerMethod {
            output,
            input: None,
            resource: None,
        }
    }
    fn add_input(&mut self, input: &Expression) -> Result<(), HCLDataError> {
        let input_header_value = input.to_header_value()?;
        self.input = Option::from(input_header_value);
        Ok(())
    }
    fn add_resource(&mut self, resource: String) {
        self.resource = Option::from(resource);
    }
    pub(crate) fn is_complete(&self) -> Result<(), HCLDataError> {
        if self.input.is_none() {
            return Err(HCLDataError::InputError(format!("Update-with-server-method is missing 'input' value, transient-structure: {:?}", self)))
        }
        if self.resource.is_none() {
            return Err(HCLDataError::InputError(format!("Update-with-server-method is missing 'resource' value, transient-structure: {:?}", self)))
        }
        Ok(())
    }
}
