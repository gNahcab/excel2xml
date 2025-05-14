use std::collections::HashMap;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::super_field::SuperField;
use crate::parse_info::domain::prop_supplement::{PropSupplement};
use crate::parse_info::domain::resource_supplement::{ResourceSupplement};
use crate::parse_xlsx::domain::dasch_value_field::{DaschValueField, FieldsWrapper};
use crate::parse_xlsx::domain::data_row::DataRow;
use crate::parse_xlsx::domain::header::Header;
use crate::parse_xlsx::domain::permissions::{Permissions};
use crate::parse_xlsx::domain::resource_data::{to_resource_data, ResourceSupplData};
use crate::parse_xlsx::errors::ExcelDataError;

pub struct Instance {
    pub id: String,
    pub label: String,
    pub iri: Option<String>,
    pub ark: Option<String>,
    pub res_permissions: Permissions,
    pub bitstream: Option<String>,
    pub bitstream_permissions: Option<Permissions>,
    pub dasch_value_fields:  Vec<DaschValueField>,
}

impl Instance {
    fn new(dasch_value_fields: Vec<DaschValueField>, resource_data: ResourceSupplData, id: String, label: String) -> Self {
        Self{
            id,
            label,
            iri: resource_data.iri,
            ark: resource_data.ark,
            res_permissions: resource_data.res_permissions,
            bitstream: resource_data.bitstream,
            bitstream_permissions: resource_data.bitstream_permissions,
            dasch_value_fields,
        }
    }
}
#[derive(Debug)]
struct TransientInstance {
    id: Option<String>,
    label: Option<String>,
    propname_to_values: HashMap<String, Vec<String>>,
    prop_name_to_prop_suppl_values: HashMap<String, Vec<(PropSupplement, Vec<String>)>>,
    res_suppl_values: Vec<(ResourceSupplement, String)>,
}


impl TransientInstance {
    fn new() -> Self {
        TransientInstance {
            id: None,
            label: None,
            propname_to_values: Default::default(),
            prop_name_to_prop_suppl_values: Default::default(),
            res_suppl_values: vec![],
        }
    }
    pub(crate) fn add_prop_suppl(&mut self, prop_suppl: PropSupplement, entries: Vec<String>) {
        if !self.prop_name_to_prop_suppl_values.contains_key(&prop_suppl.part_of) {
            self.prop_name_to_prop_suppl_values.insert(prop_suppl.part_of.to_owned(), vec![]);
        }
        let mutable_vec = self.prop_name_to_prop_suppl_values.get_mut(&prop_suppl.part_of).unwrap();
        mutable_vec.push((prop_suppl.to_owned(), entries));
    }
    pub(crate) fn add_id(&mut self, id: String) -> Result<(), ExcelDataError> {
        if self.id.is_some() {
            return Err(ExcelDataError::InputError(format!("Instance: multiple ids: First: '{}', Second: '{}'", self.id.as_ref().unwrap(),id)));
        }
        self.id = Some(id);
        Ok(())
    }
    pub(crate) fn add_label(&mut self, label: String) -> Result<(), ExcelDataError> {
        if self.label.is_some() {
            return Err(ExcelDataError::InputError(format!("Instance: multiple labels: First: '{}', Second: '{}'", self.label.as_ref().unwrap(),label)));
        }
        self.label = Some(label);
        Ok(())
    }

    fn found_id_label(&self) -> Result<(), ExcelDataError> {
        if self.label.is_none() {
            return Err(ExcelDataError::InputError(format!("Instance: Cannot find label in resource with propname_to_values: '{:?}'", self.propname_to_values)))
        }
        if self.id.is_none() {
            return Err(ExcelDataError::InputError(format!("Instance: Cannot find id in resource with propname_to_values: '{:?}'", self.propname_to_values)))
        }
        Ok(())
    }
    pub(crate) fn add_res_suppl(&mut self, res_suppl: ResourceSupplement, entry: String) {
        self.res_suppl_values.push((res_suppl, entry));
    }
    fn add_values_of_prop(&mut self, prop_name: &String, value: Vec<String>) -> Result<(), ExcelDataError> {
        if self.propname_to_values.contains_key(prop_name) {
            return Err(ExcelDataError::InputError(format!("Found multiple time same propname '{}' used as key for different value. First: '{:?}', second: '{:?}'", prop_name, value, self.propname_to_values.get(prop_name).unwrap())))
        }
        self.propname_to_values.insert(prop_name.to_owned(), value);
        Ok(())
    }
}


pub struct InstanceWrapper(pub(crate) DataRow);


impl InstanceWrapper {
    pub(crate) fn to_instance(&self, data_model: &&DataModel, separator: &String, row_nr_to_propname: &HashMap<usize, String>, row_nr_to_prop_suppl: &HashMap<usize, PropSupplement>, row_nr_to_res_suppl: &HashMap<usize, ResourceSupplement>, row_nr_to_id_label: &HashMap<usize, Header>, super_field: &SuperField) -> Result<Instance, ExcelDataError> {
        let mut transient_instance = TransientInstance::new();
        //transient_instance.add_resource_permissions(res_permissions);
        //let copyright_holder = extract_or_create_copyright_holder();
        //transient_instance.add_copyright_holder(copyright_holder);
        for (row_nr, entry) in self.0.row.iter().enumerate() {
            let entry = entry.trim();
            if entry.is_empty() {
                continue;
            }
            let entries: Vec<String> = entry.split(separator).map(|value|value.to_string()).collect();
            if row_nr_to_propname.contains_key(&row_nr) {
                let header = row_nr_to_propname.get(&row_nr).unwrap();
                transient_instance.add_values_of_prop(header, entries.to_owned())?;
            }
            if row_nr_to_prop_suppl.contains_key(&row_nr) {
                let prop_suppl = row_nr_to_prop_suppl.get(&row_nr).unwrap();
                transient_instance.add_prop_suppl(prop_suppl.to_owned(), entries.to_owned());
            }
            if row_nr_to_res_suppl.contains_key(&row_nr) {
                let res_suppl = row_nr_to_res_suppl.get(&row_nr).unwrap();
                if entries.len() != 1 {
                    return Err(ExcelDataError::InputError(format!("Entries of res-suppl should be 1, but found '{}' number of entries in :'{:?}'", entries.len(), entries)));
                }
                transient_instance.add_res_suppl(res_suppl.to_owned(), entry.to_owned());
            }
            if row_nr_to_id_label.contains_key(&row_nr) {
                match row_nr_to_id_label.get(&row_nr).unwrap() {
                    Header::ID => {
                        transient_instance.add_id(entry.to_string())?;
                    }
                    Header::Label => {
                        transient_instance.add_label(entry.to_string())?;
                    }
                    _ => {todo!("remove everything except id, label")}
                }
            } else {
                //ignore
            }
        }
        transient_instance.found_id_label()?;
        let dasch_value_fields = FieldsWrapper(transient_instance.propname_to_values.to_owned(), transient_instance.prop_name_to_prop_suppl_values.to_owned()).to_dasch_value_fields(data_model)?;
        let resource_data = to_resource_data(&transient_instance.res_suppl_values, super_field)?;
        Ok(Instance::new(dasch_value_fields, resource_data, transient_instance.id.unwrap(), transient_instance.label.unwrap()))
    }
}

fn extract_or_create_copyright_holder() -> () {
    todo!()
}

fn extract_or_create_res_permissions() -> Permissions {
    // if res_permission does exist in the header, we add it here
    // todo
    /*
    if xlsx_permissions.is_some() {
        xlsx_permissions.unwrap()
    } else if created_permissions.is_some() {
        created_permissions.unwrap()
    } else {
        Permissions::DEFAULT
    }
     */
    Permissions::DEFAULT
}

/*
fn fill_part_data_resource(data_row: &DataRow, headers: &PartDataHeader, separator: &String, data_model: &DataModel, row_nr: usize) -> Result<PartInstance, ExcelDataError> {
    let mut part_data_resource = PartInstance::new();
    add_res_prop(data_row, &mut part_data_resource, headers, row_nr)?;
    add_bitstream(data_row, &mut part_data_resource, headers, row_nr)?;
    add_propnames_and_subheaders(data_row, &mut part_data_resource, headers, separator, data_model)?;
    part_data_resource.complete(row_nr, headers)?;
    Ok(part_data_resource)
}


fn add_res_prop(data_row: &DataRow, transient_data_resource: &mut PartInstance, headers: &PartDataHeader, nr: usize) -> Result<(), ExcelDataError> {
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
fn add_bitstream(data_row: &DataRow, transient_data_resource: &mut PartInstance, headers: &PartDataHeader, nr: usize) -> Result<(), ExcelDataError> {
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

fn add_propnames_and_subheaders(data_row: &DataRow, transient_data_resource: &mut PartInstance, headers: &PartDataHeader, separator: &String, data_model: &DataModel) -> Result<(), ExcelDataError> {
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


 */
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





