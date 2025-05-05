use std::collections::HashMap;
use hcl::value;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_xlsx::domain::dasch_value_field::{DaschValueField, ValueFieldWrapper};
use crate::parse_xlsx::domain::transient_data_header::{PartDataHeader, TransientDataHeader};
use crate::parse_xlsx::domain::data_row::DataRow;
use crate::parse_xlsx::domain::permissions::{Permissions, PermissionsWrapper};
use crate::parse_xlsx::domain::header::Header;
use crate::parse_xlsx::domain::subheader_value::{subheader_value, SubheaderValues};
use crate::parse_xlsx::errors::ExcelDataError;

pub struct DataResource {
    pub id: String,
    pub label: String,
    pub iri: Option<String>,
    pub ark: Option<String>,
    pub res_permissions: Permissions,
    pub bitstream: Option<String>,
    pub bitstream_permissions: Option<Permissions>,
    pub propname_to_values: HashMap<String, DaschValueField>,
}

impl DataResource {
    fn new(transient_data_resource: PartDataResource, created_data_resource: PartDataResource, res_permissions: Permissions) -> Self {
        let id = if transient_data_resource.id.is_some() {
            transient_data_resource.id.unwrap()
        } else {
            created_data_resource.id.unwrap()
        };
        let label = if transient_data_resource.label.is_some() {
            transient_data_resource.label.unwrap()
        } else {
            created_data_resource.label.unwrap()
        };
        let iri = if transient_data_resource.iri.is_some() {
            transient_data_resource.iri
        } else {
            created_data_resource.iri
        };
        let ark = if transient_data_resource.ark.is_some() {
            transient_data_resource.ark
        } else {
            created_data_resource.ark
        };
        let bitstream_permissions = if transient_data_resource.bitstream_permissions.is_some() {
            transient_data_resource.bitstream_permissions
        } else {
            created_data_resource.bitstream_permissions
        };
        let bitstream = if transient_data_resource.bitstream.is_some() {
            transient_data_resource.bitstream
        } else {
            created_data_resource.bitstream
        };
        let mut propname_to_values = transient_data_resource.propname_to_values;
        created_data_resource.propname_to_values.iter().for_each(|(propname, value)| { propname_to_values.insert(propname.to_string(), value.to_owned()); });


        DataResource {
            id,
            label,
            iri,
            ark,
            res_permissions,
            bitstream,
            bitstream_permissions,
            propname_to_values,
        }
    }
}

pub struct DataResourceWrapper(pub(crate) (DataRow, DataRow));

impl DataResourceWrapper {
    pub(crate) fn to_data_resource(&self, data_model: &DataModel, separator: &String, headers: &TransientDataHeader, row_nr: usize) -> Result<DataResource, ExcelDataError> {
        let  xlsx_data_resource= fill_part_data_resource(&self.0.0, &headers.xlsx_data_header, separator, data_model,  row_nr)?;
        let  created_data_resource = fill_part_data_resource(&self.0.1, &headers.created_data_header, separator, data_model,  row_nr)?;
        let res_permissions = extract_or_create_res_permissions(xlsx_data_resource.res_permissions, created_data_resource.res_permissions);
        Ok(DataResource::new(xlsx_data_resource, created_data_resource, res_permissions))
    }


}

fn extract_or_create_res_permissions(xlsx_permissions: Option<Permissions>, created_permissions: Option<Permissions>) -> Permissions {
    // if res_permission does exist in the header, we add it here
    if xlsx_permissions.is_some() {
        xlsx_permissions.unwrap()
    } else if created_permissions.is_some() {
        created_permissions.unwrap()
    } else {
        Permissions::DEFAULT
    }
}

fn fill_part_data_resource(data_row: &DataRow, headers: &PartDataHeader, separator: &String, data_model: &DataModel, row_nr: usize) -> Result<PartDataResource, ExcelDataError> {
    let mut part_data_resource = PartDataResource::new();
    add_res_prop(data_row, &mut part_data_resource, headers, row_nr)?;
    add_bitstream(data_row, &mut part_data_resource, headers, row_nr)?;
    add_propnames_and_subheaders(data_row, &mut part_data_resource, headers, separator, data_model)?;
    part_data_resource.complete(row_nr, headers)?;
    Ok(part_data_resource)
}

fn add_res_prop(data_row: &DataRow, transient_data_resource: &mut PartDataResource, headers: &PartDataHeader, nr: usize) -> Result<(), ExcelDataError> {
    for (header, pos) in headers.res_prop_to_pos.iter() {
        match header {
            Header::ID => {
                let value = &data_row.row[pos.to_owned()].trim();
                if value.is_empty() {
                    return Err(ExcelDataError::ParsingError(format!("error in data_row.row-nr '{}' at position '{}'. ID-Header seems empty. Whole data_row.row: {:?}",nr, pos, data_row.row)))
                }
                transient_data_resource.add_id(value.to_string());
            }
            Header::Label => {
                let value = &data_row.row[pos.to_owned()].trim();
                if value.is_empty() {
                    return Err(ExcelDataError::ParsingError(format!("error in data_row.row-nr '{}' at position '{}'. Label-Header seems empty. Whole data_row.row: {:?}",nr, pos, data_row.row)))
                }
                transient_data_resource.add_label(value.to_string());
            }
            Header::Permissions => {
                let value = &data_row.row[pos.to_owned()].trim();
                let permissions = PermissionsWrapper(value.to_string()).to_permissions()?;
                transient_data_resource.add_resource_permissions(permissions)?
            }
            Header::ARK => {
                let value = &data_row.row[pos.to_owned()].trim();
                transient_data_resource.add_ark(value.to_string());
            }
            Header::IRI => {
                let value = &data_row.row[pos.to_owned()].trim();
                transient_data_resource.add_iri(value.to_string());
            }
            _ => {
                return Err(ExcelDataError::ParsingError(format!("Grave Error: this error should not happen. This list is not supposed to contain this header: {:?}", header)))
            }
        }
    }
    Ok(())
}
fn add_bitstream(data_row: &DataRow, transient_data_resource: &mut PartDataResource, headers: &PartDataHeader, nr: usize) -> Result<(), ExcelDataError> {
    if headers.bitstream.is_some() {
        let value = &data_row.row[headers.bitstream.unwrap()].trim();
        if value.is_empty() {
            return Err(ExcelDataError::ParsingError(format!("error in row-nr '{:?}' at position '{:?}'. Bitstream-Header seems empty. Whole row: {:?}",nr, headers.bitstream.as_ref().unwrap(), data_row.row)))
        }
        transient_data_resource.add_bitstream(value.to_string());
        if headers.bitstream_permissions.is_some() {
            let value = &data_row.row[headers.bitstream_permissions.unwrap()].trim();
            transient_data_resource.add_bitstream_permissions(value.to_string())?;
        }
    }

    Ok(())
}

fn add_propnames_and_subheaders(data_row: &DataRow, transient_data_resource: &mut PartDataResource, headers: &PartDataHeader, separator: &String, data_model: &DataModel) -> Result<(), ExcelDataError> {
    for (propname, pos ) in headers.propname_to_pos.iter() {
        let subheader = to_subheader_value(data_row, headers, propname, separator, data_model)?;
        let raw_value = &data_row.row[pos.to_owned()].trim();
        if raw_value.is_empty() {
            continue;
        }
        let values = split_field(raw_value, separator);
        let value_field: DaschValueField = ValueFieldWrapper(values).to_dasch_value_field(data_model, propname, subheader)?;
        transient_data_resource.add_values_of_prop(propname, value_field);
    }
    Ok(())
}

fn to_subheader_value(data_row: &DataRow, headers: &PartDataHeader, propname: &String, separator: &String, data_model: &DataModel) -> Result<Option<SubheaderValues>, ExcelDataError> {
    match headers.propname_to_subheader.get(propname) {
        None => {
            Ok(None)
        }
        Some(subheader) => {
            subheader_value(&data_row.row,
                            &subheader,
                            separator,
                            &data_model.properties.iter().find(|property| property.name.eq(propname)).unwrap(),
                            propname)
        }
    }
}


pub fn split_field(field: &&str, separator: &String) -> Vec<String> {
    match field.contains(separator) {
        true => {
            field.split(separator).map(|splitter| splitter.to_string()).collect()
        }
        false => {
            vec![field.to_string()]
        }
    }
}

struct TransientDataResource {
    xlsx_data_resource: PartDataResource,
    created_data_resource: PartDataResource,
}
impl TransientDataResource {
    fn new(
        xlsx_data_resource: PartDataResource,
        created_data_resource: PartDataResource,
    ) -> Self {
        TransientDataResource{ xlsx_data_resource, created_data_resource }
    }

}

struct PartDataResource {
    id: Option<String>,
    label: Option<String>,
    res_permissions: Option<Permissions>,
    iri: Option<String>,
    ark: Option<String>,
    bitstream: Option<String>,
    bitstream_permissions: Option<Permissions>,
    propname_to_values: HashMap<String, DaschValueField>,
}

impl PartDataResource {
    fn new() -> Self {
        PartDataResource {
            id: None,
            label: None,
            res_permissions: None,
            iri: None,
            ark: None,
            propname_to_values: Default::default(),
            bitstream: None,
            bitstream_permissions: None,
        }
    }
    fn add_id(&mut self, id: String)  {
        self.id = Option::from(id);
    }
    fn add_label(&mut self, label: String) {
        self.label = Option::from(label);
    }
    fn add_resource_permissions(&mut self, permissions: Permissions) -> Result<(), ExcelDataError>  {
        self.res_permissions = Some(permissions);
        Ok(())
    }
    fn add_iri(&mut self, iri: String) {
        if !iri.is_empty() {
            self.iri = Option::from(iri);
        }
    }
    fn add_ark(&mut self, ark: String) {
        if !ark.is_empty() {
            self.ark = Option::from(ark);
        }
    }
    fn add_bitstream(&mut self, bitstream: String) {
        self.bitstream = Option::from(bitstream);
    }
    fn add_bitstream_permissions(&mut self, value: String) -> Result<(), ExcelDataError> {
        self.bitstream_permissions =  Some(PermissionsWrapper(value).to_permissions()?);
        Ok(())
    }
    fn add_values_of_prop(&mut self, prop_name: &String, value: DaschValueField) {
        self.propname_to_values.insert(prop_name.to_owned(), value);
    }
    fn complete(&self, row_nr: usize, headers: &PartDataHeader) -> Result<(), ExcelDataError> {
        if self.id.is_none() && headers.id.is_some() {
            return Err(ExcelDataError::ParsingError(format!("No id found in row-nr '{}'!", row_nr)));

        }
        if self.label.is_none() && headers.label.is_some() {
            return Err(ExcelDataError::ParsingError(format!("No label found in row-nr '{}'!", row_nr)));
        }
        Ok(())
    }
}




