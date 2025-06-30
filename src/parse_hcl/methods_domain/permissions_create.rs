use hcl::Block;
use crate::expression_trait::ExpressionTransform;
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::methods_domain::wrapper_trait_block::Wrapper;
use crate::parse_xlsx::domain::permissions::Permissions;
use crate::parse_xlsx::domain::permissions::PermissionsWrapper;

pub struct WrapperPermissionsCreate(pub(crate) Block);

#[derive(Clone, Debug)]
pub struct PermissionsCreate {
    pub output: String,
    pub permissions: Permissions
}

impl PermissionsCreate {
    fn new(transient_permissions_create: TransientPermissionsCreate) -> PermissionsCreate {
        PermissionsCreate{ output: transient_permissions_create.output, permissions: transient_permissions_create.permissions.unwrap() }
    }
}

struct TransientPermissionsCreate {
    output: String,
    permissions: Option<Permissions>
}
impl TransientPermissionsCreate  {
    fn new(output: String) -> Self {
        Self{ output , permissions: None }
    }
    fn add_permissions(&mut self, permissions: Permissions) -> Result<(), HCLDataError>{
        if self.permissions.is_some() {
            return Err(HCLDataError::InputError(format!("Permissions-create-method: Multiple permissions declared. First: {:?}, second: {:?}", self.permissions.as_ref().unwrap(), permissions)));
        }
        self.permissions = Some(permissions);
        Ok(())
    }
}
impl WrapperPermissionsCreate {
    pub(crate) fn to_permissions_create_method(&self, output: String) -> Result<PermissionsCreate, HCLDataError> {
        self.0.no_blocks()?;
        let mut transient = TransientPermissionsCreate::new(output);
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "value" => {
                    let permissions = match PermissionsWrapper(attribute.expr.to_string_2()?).to_permissions() {
                        Ok(permissions) => {permissions}
                        Err(err) => {
                            return Err(HCLDataError::InputError(format!("Permissions-create-method: cannot transform permissions-string '{:?}' to Permissions.", attribute.expr)));
                        }
                    };
                    transient.add_permissions(permissions)?;
                }
                _ => {
                    return Err(HCLDataError::InputError(format!("Permissions-create-method: key should be 'value', but found: '{}'", attribute.key)));
                }
            }
        }
        Ok(PermissionsCreate::new(transient))
    }
}
