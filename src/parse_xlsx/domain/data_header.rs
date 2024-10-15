use std::collections::HashMap;
use crate::json2datamodel::domain::data_model::DataModel;
use crate::json2datamodel::domain::resource::DMResource;
use crate::parse_xlsx::domain::data_row::DataRow;
use crate::parse_xlsx::domain::data_sheet::compare_header_to_data_model;
use crate::parse_xlsx::domain::header::Header;
use crate::parse_xlsx::domain::headers::{to_headers, Headers};
use crate::parse_xlsx::domain::permissions::Permissions;
use crate::parse_xlsx::domain::subheader::{Subheader, TransientSubheader};
use crate::parse_xlsx::errors::ExcelDataError;
use crate::parse_xlsx::errors::ExcelDataError::ParsingError;

pub struct DataHeader {
    id: usize,
    label: usize,
    permissions: Option<usize>,
    iri: Option<usize>,
    ark: Option<usize>,
    bitstream: Option<usize>,
    propname_to_pos: HashMap<String, usize>,

}

impl DataHeader {
    fn new(transient_data_header: TransientDataHeader) -> DataHeader {
        DataHeader{
            id: transient_data_header.id.unwrap(),
            label: transient_data_header.label.unwrap(),
            permissions: transient_data_header.permissions,
            iri: transient_data_header.iri,
            ark: transient_data_header.ark,
            bitstream: transient_data_header.bitstream,
            propname_to_pos: transient_data_header.propname_to_pos,
        }
    }
}

struct TransientDataHeader {
    id: Option<usize>,
    label: Option<usize>,
    permissions: Option<usize>,
    iri: Option<usize>,
    ark: Option<usize>,
    bitstream: Option<usize>,
    propname_to_pos: HashMap<String, usize>,
    propname_to_subheader: HashMap<String, Subheader>
}

impl TransientDataHeader {
    fn new() -> Self {
        TransientDataHeader {
            id: None,
            label: None,
            permissions: None,
            iri: None,
            ark: None,
            bitstream: None,
            propname_to_pos: Default::default(),
            propname_to_subheader: Default::default(),
        }
    }
    pub(crate) fn add_id(&mut self,id: &String, pos: usize) -> Result<(), ExcelDataError>  {
        if self.id.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate id: {}", id) ))
        }
        self.id = Option::from(pos);
        Ok(())
    }
    pub(crate) fn add_label(&mut self, label: &String, pos: usize) -> Result<(), ExcelDataError>  {
        if self.label.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate label: {}", label) ))
        }
        self.label = Option::from(pos);
        Ok(())
    }
    pub(crate) fn add_permissions(&mut self, permissions: &Permissions, pos: usize) -> Result<(), ExcelDataError>  {
        if self.permissions.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate permissions: {:?}", permissions) ))
        }
        self.permissions = Option::from(pos);
        Ok(())
    }
    pub(crate) fn add_iri(&mut self, iri: &String, pos: usize) -> Result<(), ExcelDataError> {
        if self.iri.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate iri: {}", iri) ))
        }
        self.iri = Option::from(pos);
        Ok(())
    }
    pub(crate) fn add_ark(&mut self, ark: &String, pos: usize) -> Result<(), ExcelDataError>  {
        if self.ark.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate ark: {}", ark) ))
        }
        self.ark = Option::from(pos);
        Ok(())
    }
    pub(crate) fn add_bitstream(&mut self, bitstream: &String, pos: usize) -> Result<(), ExcelDataError> {
        if self.bitstream.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate bitstream: {}", bitstream) ))
        }
        self.bitstream = Option::from(pos);
        Ok(())
    }
    pub(crate) fn add_positions_of_prop(&mut self, prop_name: String, pos: usize) -> Result<(), ExcelDataError> {
        if self.propname_to_pos.contains_key(&prop_name) {
            return Err(ExcelDataError::InputError( format!("Duplicate prop_name found: {}", prop_name) ))
        }
        self.propname_to_pos.insert(prop_name, pos.to_owned());
        Ok(())
    }
    pub(crate) fn properties_correct(&self, data_model: &DataModel, res_name: String) -> Result<(), ExcelDataError> {
        let propnames_of_res = data_model.resources.iter().filter(|resource| resource.name == res_name).collect::<Vec<&DMResource>>().first().unwrap()
            .properties.iter().map( |res_property| res_property.propname.as_str()).collect::<Vec<&str>>();
        for (prop_name, _) in self.propname_to_pos.iter() {
            // propname must be part of resource
            if !propnames_of_res.contains(&res_name.as_str()) {
                return Err(ParsingError(format!("Property '{}' not found in Resource '{}'. All propnames: {:?}", prop_name, res_name, propnames_of_res)))
            }
        }
        Ok(())
    }
    pub(crate) fn label_id_exist(&self) -> Result<(), ExcelDataError> {
        // label, id must exist
        if self.id.is_none() {
            return Err(ExcelDataError::InputError(format!("Missing id for data resource with label: {:?}", self.label.as_ref().unwrap())));
        }
        if self.label.is_none() {
            return Err(ExcelDataError::InputError(format!("Missing label for data resource with id: {:?}", self.id.as_ref().unwrap())));
        }
        Ok(())
    }
    pub(crate) fn positions_correct(&self) -> Result<(), ExcelDataError> {
        // check that id, label, iri, ark are positioned at the beginning and all properties after
        let mut positions: Vec<usize>  = vec![&self.label, &self.id, &self.iri, &self.ark]
            .iter()
            .filter(|res_header|res_header.is_some())
            .map(|res_header|res_header.as_ref().unwrap().to_owned())
            .collect();
        positions.sort();

        let positions_propnames: Vec<&usize> = self.propname_to_pos.iter().map(|(_, pos)|pos).collect();

        if positions.last().unwrap() > positions_propnames.first().unwrap() {

            return Err(ExcelDataError::ParsingError(format!("First header of propnames '{}' is before last header of resource '{}' (id, label, ark, iri)", positions_propnames.first().unwrap(), positions.last().unwrap())));
        }
        Ok(())
    }
    pub(crate) fn add_permissions_comment_encoding(&mut self, pos_to_header: &HashMap<usize, Header>) -> Result<(), ExcelDataError> {
        // to find permissions belonging to the resource: we look between id and bitstream or ark or iri or label (depends on what exists in headers)
        // permissions, encoding, comments belonging to resources: we look if the headers after the header with the properties are called 'permissions', 'encoding' or 'comment'
        // 1 check for permissions of resource: check between id, label, ark, iri( if iri, ark exist)
        let(first, last) = first_last_of_res_prop([self.label, self.id, self.ark, self.iri]);
        self.add_permissions_of_resource(pos_to_header, first, last)?;
        // 2 check for permissions of bitstream
        self.add_permissions_of_bitstream(pos_to_header)?;
        // 3 check for permissions of properties
        self.add_permissions_of_properties(pos_to_header, last)?;
        Ok(())
    }

    fn add_permissions_of_resource(&mut self, pos_to_header: &HashMap<usize, Header>, start: usize, last: usize) -> Result<(), ExcelDataError> {
        // 1 check for permissions of resource: check between id, label, ark, iri( if iri, ark exist)
        let mut curr = start;
        while curr <= (last + 1) {
            let curr_position = pos_to_header.get(&curr);
            curr += 1;
            let header = match curr_position {
                None => {
                    return Err(ExcelDataError::ParsingError("Cannot find header, because position doesn't exist. This should never happen.".to_string()));
                }
                Some(header) => {header}
            };
            match header {
                Header::Permissions => {
                    self.permissions = Some(curr)
                }
                _=> {continue}
            }
        }
        Ok(())
    }

    fn add_permissions_of_bitstream(&mut self, pos_to_header: &HashMap<usize, Header>) -> Result<(), ExcelDataError> {
        // if there is a permissions after bitstream, add it, otherwise return
        if self.bitstream.is_none() {
            return Ok(());
        }
        let pos = self.bitstream.unwrap() + 1;
        let permissions_candidate = match pos_to_header.get(&pos) {
            None => {
                // out of range
                return Ok(());
            }
            Some(permissions_candidate) => { permissions_candidate }
        };
        match permissions_candidate {
            Header::Permissions => {
                self.permissions = Some(pos);
                Ok(())
            }
            _ => {Ok(())}
        }
    }

    fn add_permissions_of_properties(&mut self, pos_to_header: &HashMap<usize, Header>, last: usize) -> Result<(), ExcelDataError>{
        let mut curr = last + 1;
        while curr < pos_to_header.len() {
            let prop_header = pos_to_header.get(&curr).unwrap();
            match prop_header {
                Header::ProjectProp(prop_name) => {
                    loop {
                        curr += 1;
                        let header = match pos_to_header.get(&curr) {
                            None => {
                                // position was filtered out before, so it doesn't exist
                                // and we can continue
                                continue
                            }
                            Some(header) => {header}
                        };
                        let mut transient_subheader = TransientSubheader::new();
                        match header {
                            Header::Permissions => {
                                transient_subheader.add_permissions(curr, prop_name)?;
                            }
                            Header::Comment => {
                                transient_subheader.add_comment(curr, prop_name)?;
                            }
                            Header::Encoding => {
                                transient_subheader.add_encoding(curr, prop_name)?;
                            }
                            _=> {
                                if transient_subheader.has_values() {
                                    let subheader = Subheader::new(transient_subheader);
                                    self.propname_to_subheader.insert(prop_name.to_string(), subheader);
                                }
                                break;
                            }
                        }

                    }
                }
                _ => {return Err(ExcelDataError::ParsingError("grave error this should never happen".to_string()));}
            }
        }
        todo!()
    }
}
fn first_last_of_res_prop(res_props: [Option<usize>; 4]) -> (usize, usize)  {
    let mut positions:Vec<usize> = res_props.iter()
        .filter(|res_header|res_header.is_some())
        .map(|res_header|res_header.as_ref().unwrap().to_owned())
        .collect();
    positions.sort();
    (positions.first().unwrap().to_owned(), positions.last().unwrap().to_owned())
}
pub struct DataHeaderWrapper (pub(crate) DataRow);
impl DataHeaderWrapper {
    pub(crate) fn to_data_header(&self, dm_model: &DataModel, res_name: &String) -> Result<DataHeader, ExcelDataError> {
        let headers: Headers = to_headers(&self.0.rows, &dm_model.properties)?;
        compare_header_to_data_model(res_name, dm_model, &headers.pos_to_headers.values().collect())?;
        let mut transient_data_header = TransientDataHeader::new();
        for (pos, header) in headers.pos_to_headers.iter() {
            match header {
                Header::ID => {
                    let id = &self.0.rows[pos.to_owned()];
                    transient_data_header.add_id(id, pos.to_owned())?
                }
                Header::Label => {
                    let label = &self.0.rows[pos.to_owned()];
                    transient_data_header.add_label(label, pos.to_owned())?
                }
                Header::ARK => {
                    let ark = &self.0.rows[pos.to_owned()];
                    transient_data_header.add_ark(ark, pos.to_owned())?
                }
                Header::IRI => {
                    let iri = &self.0.rows[pos.to_owned()];
                    transient_data_header.add_iri(iri, pos.to_owned())?
                }
                Header::Bitstream => {
                    let bitstream = &self.0.rows[pos.to_owned()];
                    transient_data_header.add_bitstream(bitstream, pos.to_owned())?
                }
                Header::ProjectProp(prop_header) => {
                    transient_data_header.add_positions_of_prop(prop_header.to_owned(), pos.to_owned())?;
                }
                _ => {
                    // permissions, comment, encoding: cannot be processed here; we deal with them after matching all other headers
                }
            }
        }
        transient_data_header.positions_correct()?;
        transient_data_header.label_id_exist()?;
        transient_data_header.add_permissions_comment_encoding(&headers.pos_to_headers)?;
        Ok(DataHeader::new(transient_data_header))
    }

}

