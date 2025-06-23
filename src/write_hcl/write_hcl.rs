use std::path::PathBuf;
use calamine::{Data, Range};
use crate::parse_dm::domain::data_model::DataModel;
use crate::write_hcl::errors::WriteHCLError;
use crate::write_hcl::HCL_resource::{HCLResource, WrapperHCLResource};

pub fn write_hcl(xlsx_files: Vec<Vec<(String, Range<Data>)>>, datamodel: DataModel) -> Result<(), WriteHCLError> {
    let headers: Vec<Vec<String>> = map_headers(xlsx_files);
    let resources = hcl_resources(headers, &datamodel)?;
    write_resources_to_hcl(resources)?;
    Ok(())
}

fn map_headers(xlsx_files: Vec<Vec<(String, Range<Data>)>>) -> Vec<Vec<String>> {
    for xlsx_f in xlsx_files.iter() {
        println!("{:?}",xlsx_f)
    }

    todo!();
}

fn write_resources_to_hcl(hcl_resources: Vec<HCLResource>) -> Result<(), WriteHCLError>{
    todo!()
}

fn hcl_resources(xlsx_files: Vec<Vec<String>>, datamodel: &DataModel) -> Result<Vec<HCLResource>, WriteHCLError>{
    let mut hcl_resources = vec![];
    for xlsx_file in xlsx_files {
        let hcl_resource: HCLResource = WrapperHCLResource(xlsx_file).to_hcl_resource(&datamodel)?;
        hcl_resources.push(hcl_resource);
    }
    Ok(hcl_resources)
}