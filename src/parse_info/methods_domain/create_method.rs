use std::fmt::Debug;
use hcl::{Block};
use crate::expression_trait::ExpressionTransform;
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::methods_domain::create_loop::{Create, WrapperCreateLoop};
use crate::parse_info::methods_domain::integer_create::{IntegerCreate, WrapperIntegerCreate};
use crate::parse_info::methods_domain::permissions_create::{PermissionsCreate, WrapperPermissionsCreate};
use crate::parse_info::methods_domain::wrapper_trait_block::Wrapper;

pub struct WrapperCreateMethod(pub(crate) Block);

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
