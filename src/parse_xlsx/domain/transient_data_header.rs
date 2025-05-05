use std::collections::{HashMap, HashSet};
use std::f32::consts::E;
use std::hash::Hash;
use crate::parse_dm::domain::cardinality::Cardinality;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::property::Property;
use crate::parse_dm::domain::resource::DMResource;
use crate::parse_xlsx::domain::data_row::DataRow;
use crate::parse_xlsx::domain::header::{Extractor, Header, HeaderWrapper};
use crate::parse_xlsx::domain::subheader::{Subheader, TransientSubheader};
use crate::parse_xlsx::errors::ExcelDataError;

pub struct TransientDataHeader{
    pub xlsx_data_header: PartDataHeader,
    pub created_data_header: PartDataHeader
}

impl TransientDataHeader {
    fn new(xlsx_data_header: PartDataHeader, created_data_header: PartDataHeader) -> Self {
        TransientDataHeader{ xlsx_data_header, created_data_header }
    }
    pub(crate) fn check(&self) -> Result<(), ExcelDataError> {
        self.label_id_exist()
    }

    fn label_id_exist(&self) -> Result<(), ExcelDataError> {
        if self.xlsx_data_header.id.is_some() && self.created_data_header.id.is_some() {
            return Err(ExcelDataError::ParsingError(format!("Found two ids: one in xlsx and one created in resource with xlsx: {:?}", self.xlsx_data_header.propname_to_pos.values())))
        }
        if self.xlsx_data_header.label.is_some() && self.created_data_header.label.is_some() {
            return Err(ExcelDataError::ParsingError(format!("Found two labels: one in xlsx and one created in resource with xlsx: {:?}", self.xlsx_data_header.propname_to_pos.values())))
        }
        if self.xlsx_data_header.id.is_none() && self.created_data_header.id.is_none() {
            return Err(ExcelDataError::ParsingError(format!("Found no ids: neither in xlsx nor created in resource with xlsx: {:?}", self.xlsx_data_header.propname_to_pos.values())))
        }
        if self.xlsx_data_header.label.is_none() && self.created_data_header.label.is_none() {
            return Err(ExcelDataError::ParsingError(format!("Found no labels: neither in xlsx nor created in resource with xlsx: {:?}", self.xlsx_data_header.propname_to_pos.values())))
        }
        Ok(())
    }
}


#[derive(Debug)]
pub struct PartDataHeader {
    id: Option<usize>,
    label: Option<usize>,
    pub(crate) bitstream: Option<usize>,
    pub(crate) bitstream_permissions: Option<usize>,
    pub(crate) propname_to_pos: HashMap<String, usize>,
    pub(crate) propname_to_subheader: HashMap<String, Subheader>,
    pub(crate) res_prop_to_pos: HashMap<Header, usize>,
}

impl PartDataHeader {
    pub(crate) fn add_propname(&mut self, propname: String, pos: usize) -> Result<(), ExcelDataError> {
        if self.propname_to_pos.contains_key(&propname) {
            return Err(ExcelDataError::ParsingError(format!("found duplicate propname in headers: '{}'", propname)));
        }
        self.propname_to_pos.insert(propname, pos);
        Ok(())
    }
}

impl PartDataHeader {
    fn new() -> Self {
        PartDataHeader {
            id: None,
            label: None,
            bitstream: None,
            bitstream_permissions: None,
            propname_to_pos: Default::default(),
            propname_to_subheader: Default::default(),
            res_prop_to_pos: Default::default(),
        }
    }
    fn positions_correct(&self, last: &usize) -> Result<(), ExcelDataError> {
        // check that id, label, iri, ark are positioned at the beginning and all properties after
        let positions_propnames: Vec<&usize> = self.propname_to_pos.values().collect();

        if last > positions_propnames.first().unwrap() {
            return Err(ExcelDataError::ParsingError(format!("First header of propnames '{}' is before last header of resource '{}' (id, label, ark, iri)", positions_propnames.first().unwrap(), last)));
        }
        Ok(())
    }


}
pub struct TransientDataHeaderWrapper(pub(crate) (DataRow, DataRow));


impl TransientDataHeaderWrapper {
    pub(crate) fn to_transient_data_header(&self, dm_model: &DataModel, res_name: &String) -> Result<TransientDataHeader, ExcelDataError> {
        let (pos_to_special_prop_xlsx, pos_to_propname_xlsx) = headers_special_and_propnames(&self.0.0.row,  &dm_model.properties)?;
        let (pos_to_special_prop_created, pos_to_propname_created) = headers_special_and_propnames(&self.0.1.row,  &dm_model.properties)?;
        let resource = match dm_model.resources.iter().find(|resource| resource.name.eq(res_name)) {
            None => { return Err(ExcelDataError::ParsingError(format!("not found resource with name '{}' in data-model with resources: {:?}", res_name, dm_model.resources.iter().map(|resource|&resource.name).collect::<Vec<_>>()))) }
            Some(dm_resource) => { dm_resource }
        };
        let bitstream = extract_bitstream(&pos_to_special_prop_xlsx, &pos_to_special_prop_created)?;
        compare_header_to_data_model(
            res_name,
            &dm_model.resources,
            &pos_to_propname_xlsx.values().collect(),
            &pos_to_propname_created.values().collect(),
            bitstream
        )?;
        compare_header_to_res_prop(res_name, &pos_to_special_prop_xlsx.values().collect(), &pos_to_special_prop_created.values().collect())?;

        let mut part_data_header_xlsx = PartDataHeader::new();
        let mut part_data_header_created = PartDataHeader::new();
        fill_part_header(&mut part_data_header_xlsx, &pos_to_propname_xlsx, &pos_to_special_prop_xlsx, resource)?;
        fill_part_header(&mut part_data_header_created, &pos_to_propname_created, &pos_to_special_prop_created, resource)?;

        let transient_data_header = TransientDataHeader::new(part_data_header_xlsx, part_data_header_created);
        transient_data_header.check()?;

        Ok(transient_data_header)
    }
}


fn fill_part_header(transient_data_header: &mut PartDataHeader, pos_to_propname: &HashMap<usize, String>, pos_to_special_prop: &HashMap<usize, Header>, resource: &DMResource) -> Result<(), ExcelDataError>{
    add_propnames(transient_data_header, &pos_to_propname, resource)?;
    add_props_of_res(transient_data_header, &pos_to_special_prop)?;
    add_permissions_comment_encoding(transient_data_header, &pos_to_special_prop, &pos_to_propname)?;
    Ok(())
}

fn extract_bitstream<'a>(pos_to_special_prop_xlsx: &'a HashMap<usize, Header>, pos_to_special_prop_created: &'a HashMap<usize, Header>) -> Result<Option<&'a Header>, ExcelDataError> {
    let bitstream_xlsx = pos_to_special_prop_xlsx.values().find(|header| header == &&Header::Bitstream);
    let bitstream_created = pos_to_special_prop_created.values().find(|header| header == &&Header::Bitstream);
    if bitstream_xlsx.is_some() && bitstream_created.is_some() {
        return Err(ExcelDataError::ParsingError("Found 2 bitstreams, one in xlsx and one was created..".to_string()));
    }
    if bitstream_created.is_some() {
        Ok(bitstream_created)
    } else if bitstream_xlsx.is_some() {
        Ok(bitstream_xlsx)
    } else {
        Ok(None)
    }


}

fn compare_header_to_data_model(res_name: &String, dm_resources: &Vec<DMResource>, prop_names_xlsx: &Vec<&String>, prop_names_created: &Vec<&String>, bitstream: Option<&Header>) -> Result<(), ExcelDataError> {
    let resource = match dm_resources.iter().find(|resource| resource.name.eq(res_name)) {
        None => { return Err(ExcelDataError::ParsingError(format!("not found resource with name '{}' in data-model", res_name))) }
        Some(dm_resource) => { dm_resource }
    };
    let propnames_xlsx: Vec<String> = prop_names_xlsx.iter().map(|propname|propname.to_lowercase()).collect();
    let propnames_created: Vec<String> = prop_names_created.iter().map(|propname|propname.to_lowercase()).collect();
    let missing_propnames:Vec<_> = resource.properties.iter().map(|property|property.propname.to_lowercase()).filter(|propname| !propnames_xlsx.contains(propname) && !propnames_created.contains(propname)).collect();
    let missing_required_propnames = filter_required_propnames(missing_propnames, resource);

    if missing_required_propnames.len() != 0 {
        return Err(ExcelDataError::ParsingError(format!("not found all required propnames. Missing required propnames: {:?}. EXISTING HEADERS from xlsx: {:?}. . EXISTING HEADERS from created: {:?}", missing_required_propnames, propnames_xlsx, propnames_created)))
    }
    if resource.super_field.ends_with("Representation") && bitstream.is_none() {
        return Err(ExcelDataError::ParsingError("Resource is a 'Representation' but not found bitstream-header in headers.".to_string()))
    }
    Ok(())
}

fn filter_required_propnames(propnames: Vec<String>, dmresource: &DMResource) -> Vec<String> {
    let required:Vec<String>  = dmresource.properties.iter().filter_map(|res_prop| match res_prop.cardinality {
        Cardinality::ZeroToN => { None }
        Cardinality::ZeroToOne => { None }
        Cardinality::One => { Some(res_prop.propname.to_owned()) }
        Cardinality::OneToN => {Some(res_prop.propname.to_owned()) }
    }).collect();
    propnames.iter().map(|name| name.to_owned() ).filter(|name| required.contains(name)).collect()
}

fn compare_header_to_res_prop(res_name: &String, res_prop_values_xlsx: &Vec<&Header>, res_prop_values_created: &Vec<&Header>) -> Result<(), ExcelDataError> {
    let should_have = [Header::ID, Header::Label];
    let doesnt_have = should_have.iter().filter(|header| !res_prop_values_xlsx.contains(header) && !res_prop_values_created.contains(header)).collect::<Vec<_>>();
    if !doesnt_have.is_empty() {
        return Err(ExcelDataError::ParsingError(format!("For resource '{}'cannot find: {:?} in header, but it is mandatory.", res_name, doesnt_have)));
    }
    Ok(())
}

fn add_permission_of_resource(transient_data_header:  &mut PartDataHeader, pos_to_special_prop: &HashMap<usize, Header>) -> Result<(), ExcelDataError> {
    // 1 check for permissions of resource: check between id, label, ark, iri( if iri, ark exist) + 1
    let mut curr: usize = transient_data_header.res_prop_to_pos.values().collect::<Vec<_>>().first().unwrap().to_owned().to_owned();
    let last: _ = transient_data_header.res_prop_to_pos.values().collect::<Vec<_>>().last().unwrap().to_owned().to_owned();
    let mut permission: Option<(Header, usize)> = Option::None;
    while curr <= (last + 1) {
        let header = match pos_to_special_prop.get(&curr) {
            None => {
                // position was filtered out before, so it doesn't exist
                // and we can continue by adding +1 to curr
                curr += 1;
                continue;
            }
            Some(header) => {header}
        };
        curr += 1;
        match header {
            Header::Permissions => {
                if transient_data_header.bitstream_permissions.is_some() && transient_data_header.bitstream_permissions.unwrap() == curr {
                    continue;
                }
                if permission.is_some() {
                    return Err(ExcelDataError::ParsingError("found multiple permissions in header-part reserved for resource-properties".to_string()));
                }
                permission = Option::from((header.to_owned(), curr));
            }
            _=> {continue;}
        }
    }
    if permission.is_some() {
        // no old key was present with no old value, so the output will be None
        let _ = &transient_data_header.res_prop_to_pos.insert(permission.as_ref().unwrap().to_owned().0, permission.as_ref().unwrap().to_owned().1);
    }
    Ok(())
}

fn headers_special_and_propnames(raw_header: &Vec<String>, properties: &Vec<Property>) -> Result<(HashMap<usize, Header>, HashMap<usize, String>), ExcelDataError> {
    let(pos_to_propname, pos_to_special_props) = discern_special_headers_to_propnames(raw_header, properties)?;
    no_duplicates(pos_to_propname.values().collect())?;
    no_duplicates(pos_to_special_props.values().collect())?;
    Ok((pos_to_special_props, pos_to_propname))
}

fn discern_special_headers_to_propnames(raw_header: &Vec<String>, properties: &Vec<Property>) -> Result<(HashMap<usize, String>, HashMap<usize, Header>), ExcelDataError> {
    let mut pos_to_propname: HashMap<usize, String> = HashMap::new();
    let mut pos_to_special_props: HashMap<usize, Header> = HashMap::new();
    for (pos, raw_header) in raw_header.iter().enumerate() {
        let header = match HeaderWrapper(raw_header.to_owned()).to_header(&properties) {
            Ok(header) => {header}
            Err(_) => {
                //todo: for now ignore headers that don't exist, later require specifying this in hcl?
                continue
            }
        };
        if matches!(header, Header::ProjectProp(_)) {
            pos_to_propname.insert(pos, header.extract_value()?);}
        else {
            pos_to_special_props.insert(pos, header.to_owned());
        }
    }
    Ok((pos_to_propname, pos_to_special_props))
}

fn add_props_of_res(mut transient_data_header: &mut PartDataHeader, pos_to_special_prop: &HashMap<usize, Header>) -> Result<(), ExcelDataError> {
    for (pos, header) in pos_to_special_prop.iter() {
        match header {
            Header::ARK | Header::IRI  => {
                transient_data_header.res_prop_to_pos.insert(header.to_owned(), pos.to_owned());
            },
            Header::Bitstream => {
                transient_data_header.bitstream = Some(pos.to_owned());
            },
            Header::ID => {
                transient_data_header.id = Some(pos.to_owned());
                transient_data_header.res_prop_to_pos.insert(header.to_owned(), pos.to_owned());
            },
            Header::Label => {
                transient_data_header.label = Some(pos.to_owned());
                transient_data_header.res_prop_to_pos.insert(header.to_owned(), pos.to_owned());
            },
            _ => {
                // permissions, comment, encoding: cannot be processed here; we deal with them after matching all other headers
            },
        }
    }

    add_permissions_of_res_bitstream(transient_data_header, pos_to_special_prop);
    add_permission_of_resource(&mut transient_data_header, &pos_to_special_prop)?;
    Ok(())
}
pub(crate) fn add_permissions_of_res_bitstream(transient_header: &mut PartDataHeader, pos_to_special_prop : &HashMap<usize, Header>) {
    let pos = match pos_to_special_prop.iter().filter(|(_, header)| matches!(header, Header::Bitstream)).collect::<Vec<(&usize, &Header)>>().first() {
        None => {
            // bitstream not found
            return;
        }
        Some((pos, _)) => {pos.to_owned().to_owned()}
    };
    // if hashmap contains a key pos+1 and this value is a permissions-header, we are sure that this permissions belongs to bitstream; otherwise return
    let next_pos = pos + 1;
    let candidate = pos_to_special_prop.get(&next_pos);
    match candidate {
        None => {
            return;
        }
        Some(candidate) => {
            match candidate {
                Header::Permissions => {
                    transient_header.bitstream_permissions = Some(next_pos);
                }
                _ => {
                    // next is not permissions: return
                    return;
                }
            }
        }
    }
}
fn add_propnames(transient_data_header: &mut PartDataHeader, pos_to_propname: &HashMap<usize, String>, resource: &DMResource) -> Result<(), ExcelDataError> {
    let propnames_dm: Vec<_> = resource.properties.iter().map(|prop|&prop.propname).collect();
    _add_propnames(transient_data_header, pos_to_propname, &propnames_dm, resource.name.as_str())?;
    Ok(())
}

fn _add_propnames(transient_data_header: &mut PartDataHeader, pos_to_propname: &HashMap<usize, String>, propnames_dm: &Vec<&String>, res_name: &str) -> Result<(), ExcelDataError>{
    for (pos, propname) in pos_to_propname.iter() {
        if !propnames_dm.contains(&propname) {
            return Err(ExcelDataError::ParsingError(format!("Propname '{}' is a propname, but it is not part of resource '{}'", propname, res_name)));
        }
        transient_data_header.add_propname(propname.to_owned(), pos.to_owned())?;
    }
    Ok(())
}

pub(crate) fn add_permissions_comment_encoding(transient_data_header: &mut PartDataHeader, pos_to_special_prop: &HashMap<usize, Header>, pos_to_propname: &HashMap<usize, String>) -> Result<(), ExcelDataError> {
    // to find permissions belonging to the resource: we look between id and bitstream or ark or iri or label (depends on what exists in headers)
    // permissions, encoding, comments belonging to resources: we look if the headers after the header with the properties are called 'permissions', 'encoding' or 'comment'

    // 1 check for permissions of resource: check between id, label, ark, iri( if iri, ark exist)
    /*
    let last = transient_data_header.res_prop_to_pos.values().last().unwrap();
    transient_data_header.positions_correct(last)?;

     */
    // 3 check for permissions of properties
    add_permissions_of_properties(transient_data_header, pos_to_special_prop, pos_to_propname)?;
    Ok(())
}
fn add_permissions_of_properties(transient_data_header: &mut PartDataHeader, pos_to_special_header: &HashMap<usize, Header>, pos_to_propname: &HashMap<usize, String>) -> Result<(), ExcelDataError>{
    let mut positions_special_prop: Vec<_> = pos_to_special_header.keys().collect::<Vec<_>>();
    positions_special_prop.sort();
    for (pos, prop_name) in pos_to_propname.iter() {
        let mut curr = pos.to_owned();
        let mut transient_subheader = TransientSubheader::new();
        loop {
            curr += 1;
            if pos_to_propname.contains_key(&curr) {
                // curr is a propname, so we won't find any permissions etc. for the propname before
                break
            }
            if positions_special_prop.last().unwrap() < &&curr {
                // last special prop passed, we would loop into infinity otherwise
                break
            }
            let header = match pos_to_special_header.get(&curr) {
                None => {
                    // position was filtered out before, so it doesn't exist
                    // and we can continue by adding +1 to curr
                    continue
                }
                Some(header) => {header}
            };
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
                    // if not encoding, permissions, comment
                    break;
                }
            }
        }
        if transient_subheader.has_values() {
            let subheader = Subheader::new(transient_subheader.permissions, transient_subheader.encoding, transient_subheader.comment);
            transient_data_header.propname_to_subheader.insert(prop_name.to_string(), subheader);
        }
    }
    Ok(())
}
fn no_duplicates<T>(headers: Vec<T>) -> Result<(), ExcelDataError>
where T: Hash + ToOwned + std::fmt::Debug + Copy, T: std::cmp::Eq
{
    let mut hash_set: HashSet<T> = HashSet::new();
    for header in headers {
        if !hash_set.insert(header) {
            return Err(ExcelDataError::InputError(format!("found duplicate in headers: {:?}", header)));
        }
    }
    Ok(())
}

