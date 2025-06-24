use crate::parse_dm::domain::data_model::DataModel;
use crate::write_hcl::errors::WriteHCLError;

pub struct HCLResource {

}
pub struct WrapperHCLResource(pub(crate) (String, String, Vec<String>));

impl WrapperHCLResource {
    pub fn to_hcl_resource(self, data_model: &DataModel) -> Result<HCLResource, WriteHCLError> {
        let res_names: Vec<String> = data_model.resources.iter().map(|dm_res|dm_res.name.to_owned()).collect();
        todo!();

    }
}

