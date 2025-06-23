use calamine::{Data, Range};
use crate::parse_dm::domain::data_model::DataModel;
use crate::write_hcl::errors::WriteHCLError;

pub struct HCLResource {

}
pub struct WrapperHCLResource(pub(crate) Vec<String>);

impl WrapperHCLResource {
    pub fn to_hcl_resource(self, data_model: &DataModel) -> Result<HCLResource, WriteHCLError> {
        println!("{:?}", self.0);
        todo!()
    }
}

