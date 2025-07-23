use std::collections::HashMap;
use std::path::PathBuf;
use calamine::{Data, Range};
use crate::parse_dm::domain::data_model::DataModel;
use crate::write_hcl::errors::WriteHCLError;
use crate::write_hcl::hcl_resource::{HCLResource, WrapperHCLResource};

pub fn write_hcl(file_name_table_name_table_headers: Vec<(String, String, Vec<String>)>, datamodel: DataModel, dm_path: &PathBuf) -> Result<(), WriteHCLError> {
    let resources = hcl_resources(file_name_table_name_table_headers, &datamodel, dm_path)?;
    write_resources_to_hcl(resources)?;
    Ok(())
}


fn write_resources_to_hcl(hcl_resources: Vec<HCLResource>) -> Result<(), WriteHCLError>{
    todo!()
}

fn hcl_resources(file_name_table_name_table_headers: Vec<(String, String, Vec<String>)>, datamodel: &DataModel, dm_path: &PathBuf) -> Result<Vec<HCLResource>, WriteHCLError>{
    let mut hcl_resources = vec![];
    for filename_table_name_headers in file_name_table_name_table_headers {
        let hcl_resource: HCLResource = WrapperHCLResource(filename_table_name_headers).to_hcl_resource(&datamodel, dm_path)?;
        hcl_resources.push(hcl_resource);
    }
    Ok(hcl_resources)
}