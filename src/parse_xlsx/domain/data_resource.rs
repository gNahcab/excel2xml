use std::collections::HashMap;
use crate::json2datamodel::domain::data_model::DataModel;
use crate::parse_xlsx::domain::dasch_value::{ValueField, ValueFieldWrapper};
use crate::parse_xlsx::domain::data_header::DataHeader;
use crate::parse_xlsx::domain::data_row::DataRow;
use crate::parse_xlsx::domain::encoding::{Encoding, EncodingWrapper};
use crate::parse_xlsx::domain::permissions::{Permissions, PermissionsWrapper};
use crate::parse_xlsx::domain::header::Header;
use crate::parse_xlsx::domain::subheader_value::{SubheaderValues, TransientSubheaderValues};
use crate::parse_xlsx::errors::ExcelDataError;

pub struct DataResource {
    id: String,
    label: String,
    iri: Option<String>,
    ark: Option<String>,
    res_permissions: Option<Permissions>,
    bitstream: Option<String>,
    bitstream_permissions: Option<Permissions>,
    propname_to_values: HashMap<String, ValueField>,
    propname_to_subheader: HashMap<String, SubheaderValues>
}

impl DataResource {
    fn new(transient_data_resource: TransientDataResource) -> Self {

        DataResource {
            id: transient_data_resource.id.unwrap(),
            label: transient_data_resource.label.unwrap(),
            iri: transient_data_resource.iri,
            ark: transient_data_resource.ark,
            res_permissions: transient_data_resource.res_permissions,
            bitstream: transient_data_resource.bitstream,
            bitstream_permissions: transient_data_resource.bitstream_permissions,
            propname_to_values: transient_data_resource.propname_to_values,
            propname_to_subheader: transient_data_resource.propname_to_subheader,
        }
    }
}

pub struct DataResourceWrapper(pub(crate) DataRow);

impl DataResourceWrapper {
    pub(crate) fn to_data_resource(&self, data_model: &DataModel, separator: &String, res_name: &String, headers: &DataHeader, row_nr: usize) -> Result<DataResource, ExcelDataError> {
        let mut transient_data_resource = TransientDataResource::new();
        self.add_res_prop(&mut transient_data_resource, headers, row_nr)?;
        self.add_bitstream(&mut transient_data_resource, headers, row_nr)?;
        self.add_propnames_and_subheaders(&mut transient_data_resource, headers, separator, data_model, row_nr)?;
        Ok(DataResource::new(transient_data_resource))
    }

    fn add_res_prop(&self, transient_data_resource: &mut TransientDataResource, headers: &DataHeader, nr: usize) -> Result<(), ExcelDataError> {
        for (header, pos) in headers.res_prop_to_pos.iter() {
            match header {
                Header::ID => {
                    let value = &self.0.rows[pos.to_owned()].trim();
                    if value.is_empty() {
                        return Err(ExcelDataError::ParsingError(format!("error in row-nr '{}' at position '{}'. ID-Header seems empty. Whole row: {:?}",nr, pos, self.0.rows)))
                    }
                    transient_data_resource.add_id(value.to_string());
                }
                Header::Label => {
                    let value = &self.0.rows[pos.to_owned()].trim();
                    if value.is_empty() {
                        return Err(ExcelDataError::ParsingError(format!("error in row-nr '{}' at position '{}'. Label-Header seems empty. Whole row: {:?}",nr, pos, self.0.rows)))
                    }
                    transient_data_resource.add_label(value.to_string());
                }
                Header::Permissions => {
                    let value = &self.0.rows[pos.to_owned()].trim();
                    transient_data_resource.add_resource_permissions(value.to_string())?
                }
                Header::ARK => {
                    let value = &self.0.rows[pos.to_owned()].trim();
                    transient_data_resource.add_ark(value.to_string());
                }
                Header::IRI => {
                    let value = &self.0.rows[pos.to_owned()].trim();
                    transient_data_resource.add_iri(value.to_string());
                }
                _ => {
                    return Err(ExcelDataError::ParsingError(format!("Grave Error: this error should not happen. This list is not supposed to contain this header: {:?}", header)))
                }
            }
        }
        Ok(())
    }

    fn add_bitstream(&self, transient_data_resource: &mut TransientDataResource, headers: &DataHeader, nr: usize) -> Result<(), ExcelDataError> {
        if headers.bitstream.is_some() {
            let value = &self.0.rows[headers.bitstream.unwrap()].trim();
            if value.is_empty() {
                return Err(ExcelDataError::ParsingError(format!("error in row-nr '{}' at position '{}'. Bitstream-Header seems empty. Whole row: {:?}",nr, headers.bitstream.unwrap(), self.0.rows)))
            }
            transient_data_resource.add_bitstream(value.to_string());
            if headers.bitstream_permissions.is_some() {
                let value = &self.0.rows[headers.bitstream.unwrap()].trim();
                    transient_data_resource.add_bitstream_permissions(value.to_string());
            }
        }
        Ok(())
    }

    fn add_propnames_and_subheaders(&self, transient_data_resource: &mut TransientDataResource, headers: &DataHeader, separator: &String, data_model: &DataModel, row_nr: usize) -> Result<(), ExcelDataError> {
        for (propname, pos ) in headers.propname_to_pos.iter() {
            let raw_value = &self.0.rows[pos.to_owned()].trim();
            let values = split_field(raw_value, separator);
            let value_field: ValueField = ValueFieldWrapper(values).to_value_field(data_model, propname)?;
            transient_data_resource.add_values_of_prop(propname, value_field);
            match headers.propname_to_subheader.get(propname) {
                None => {
                    // do nothing
                }
                Some(subheader) => {
                    let mut transient_subheader_value: TransientSubheaderValues = TransientSubheaderValues::new();
                    if subheader.permissions.is_some() {
                        let raw_value = &self.0.rows[subheader.permissions.unwrap()].trim();
                        let values = split_field(raw_value, separator);
                        if !values.is_empty() {
                            let mut permissions = vec![];
                            for value in values {
                                permissions.push(PermissionsWrapper(value).to_permissions()?);
                            }
                            transient_subheader_value.add_permissions(permissions);
                        }
                    }
                    if subheader.encoding.is_some() {
                        let raw_value = &self.0.rows[subheader.encoding.unwrap()].trim();
                        let values = split_field(raw_value, separator);
                        if !values.is_empty() {
                            let mut encodings: Vec<Encoding> = vec![];
                            for value in values {
                                encodings.push(EncodingWrapper(value).to_encoding()?);
                            }
                            transient_subheader_value.add_encodings(encodings);
                        }

                    }
                    if subheader.comment.is_some() {
                        let raw_value = &self.0.rows[subheader.encoding.unwrap()].trim();
                        let values = split_field(raw_value, separator);
                        if !values.is_empty() {
                            transient_subheader_value.add_comments(values);
                        }
                    }
                    if !transient_subheader_value.is_empty() {
                        transient_data_resource.add_subheader_value(propname.to_owned(), SubheaderValues::new(transient_subheader_value));
                    }
                }
            }
        }
        Ok(())
    }
}

fn split_field(field: &&str, separator: &String) -> Vec<String> {
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
    id: Option<String>,
    label: Option<String>,
    res_permissions: Option<Permissions>,
    iri: Option<String>,
    ark: Option<String>,
    bitstream: Option<String>,
    bitstream_permissions: Option<Permissions>,
    propname_to_values: HashMap<String, ValueField>,
    propname_to_subheader: HashMap<String, SubheaderValues>,
}

impl TransientDataResource {
    fn new() -> Self {
        TransientDataResource{
            id: None,
            label: None,
            res_permissions: None,
            iri: None,
            ark: None,
            propname_to_values: Default::default(),
            bitstream: None,
            bitstream_permissions: None,
            propname_to_subheader: Default::default(),
        }
    }
    pub(crate) fn add_id(&mut self, id: String)  {
        self.id = Option::from(id);
    }
    pub(crate) fn add_label(&mut self, label: String) {
        self.label = Option::from(label);
    }
    pub(crate) fn add_resource_permissions(&mut self, permissions: String) -> Result<(), ExcelDataError>  {
        self.res_permissions = Some(PermissionsWrapper(permissions).to_permissions()?);
        Ok(())
    }
    pub(crate) fn add_iri(&mut self, iri: String) {
        if !iri.is_empty() {
            self.iri = Option::from(iri);
        }
    }
    pub(crate) fn add_ark(&mut self, ark: String) {
        if !ark.is_empty() {
            self.ark = Option::from(ark);
        }
    }
    pub(crate) fn add_bitstream(&mut self, bitstream: String) {
        self.bitstream = Option::from(bitstream);
    }
    pub(crate) fn add_bitstream_permissions(&mut self, value: String) -> Result<(), ExcelDataError> {
        self.bitstream_permissions =  Some(PermissionsWrapper(value).to_permissions()?);
        Ok(())
    }
    pub(crate) fn add_values_of_prop(&mut self, prop_name: &String, value: ValueField) {
        self.propname_to_values.insert(prop_name.to_owned(), value);
    }
    pub(crate) fn add_subheader_value(&mut self, propname: String, subheader_values: SubheaderValues) {
        self.propname_to_subheader.insert(propname, subheader_values);
    }
}




