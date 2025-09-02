use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::parse_dm::domain::cardinality::Cardinality;
use crate::parse_dm::domain::property::Property;
use crate::parse_dm::domain::resource::DMResource;
use crate::parse_hcl::domain::supplements::{Supplements};
use crate::parse_hcl::domain::prop_supplement::PropSupplement;
use crate::parse_hcl::domain::resource_supplement::{ResourceSupplType, ResourceSupplement};
use crate::parse_xlsx::domain::header::Header;
use crate::parse_xlsx::domain::transient_data_header::TransientDataHeader;
use crate::parse_xlsx::errors::ExcelDataError;

use crate::parse_xlsx::domain::hashmap_wrapper::Wrapper;
#[derive(Debug)]
pub struct DataHeader {
    pub id: usize,
    pub label: usize,
    pub res_permission: Option<usize>,
    pub bitstream: Option<usize>,
    pub bitstream_permissions: Option<usize>,
    pub propname_to_pos: HashMap<String, usize>,
    pub(crate) propname_to_pos_prop_supplement: HashMap<String, Vec<(usize, PropSupplement)>>,
}

pub(crate) struct DataHeaderWrapper(pub(crate) HashMap<String, usize>);

impl DataHeaderWrapper {
    pub(crate) fn to_data_header(&self, resource: &DMResource, row_nr_to_propname: &HashMap<usize, Vec<String>>, row_nr_to_prop_suppl: &HashMap<usize, Vec<PropSupplement>>, row_nr_to_res_suppl: &HashMap<usize, Vec<ResourceSupplement>>, row_nr_to_id_label: &HashMap<usize, Vec<Header>>) -> Result<DataHeader, ExcelDataError> {
        println!("row_nr_to_id_label: {:?}",row_nr_to_id_label);
        let mut transient_data_header = TransientDataHeader::new();
        for (pos, id_label) in row_nr_to_id_label {
            for idorlabel in id_label {
                match idorlabel {
                    Header::ID => {
                        transient_data_header.add_id_pos(pos.to_owned())?;
                    }
                    Header::Label => {
                        transient_data_header.add_label_pos(pos.to_owned())?;
                    }
                }

            }
        }
        add_propnames(&mut transient_data_header, &row_nr_to_propname, resource)?;
        add_props_of_res(&mut transient_data_header, &row_nr_to_res_suppl)?;
        add_prop_suppl(&mut transient_data_header, &row_nr_to_prop_suppl);
        transient_data_header.is_complete(&resource.super_field, &resource.name)?;
        Ok(DataHeader::new(transient_data_header))
    }
}

impl DataHeader {
    pub(crate) fn new(transient_data_header: TransientDataHeader) -> Self {
        DataHeader{
            id: transient_data_header.id.unwrap(),
            label: transient_data_header.label.unwrap(),
            res_permission: transient_data_header.res_permissions,
            bitstream: transient_data_header.bitstream,
            bitstream_permissions: transient_data_header.bitstream_permissions,
            propname_to_pos: transient_data_header.propname_to_pos,
            propname_to_pos_prop_supplement: transient_data_header.propname_to_pos_prop_supplement,
        }
    }
}

pub fn discern_label_id_propnames_and_supplements(header_to_col_nr: &HashMap<String, usize>, properties: &Vec<Property>, supplements: Option<&Supplements>) -> Result<(HashMap<usize, Vec<String>>, HashMap<usize, Vec<PropSupplement>>, HashMap<usize, Vec<ResourceSupplement>>, HashMap<usize, Vec<Header>>), ExcelDataError> {
    println!("header_to_col_nr: {:?}", header_to_col_nr);
    let mut col_to_propname: HashMap<usize, Vec<String>> = HashMap::new();
    let mut col_to_prop_suppl: HashMap<usize, Vec<PropSupplement>> = HashMap::new();
    let mut col_to_res_suppl: HashMap<usize, Vec<ResourceSupplement>> = HashMap::new();
    let mut col_to_id_label: HashMap<usize, Vec<Header>> = HashMap::new();

    let id_label = ["id", "label"];
    let propnames: Vec<&String> = properties.iter().map(|property|&property.name).collect();
    for (raw_header, pos) in header_to_col_nr.iter() {
        // map raw_headers to header
        if supplements.is_some() {
            match supplements.unwrap().header_to_res_suppl.get(raw_header) {
                None => {}
                Some(res_suppl) => {
                    col_to_res_suppl.insert_or_append(pos, res_suppl.to_owned());
                    continue;
                }
            }
            match supplements.unwrap().header_to_prop_suppl.get(raw_header) {
                None => {}
                Some(prop_suppl) => {
                    col_to_prop_suppl.insert_or_append(pos, prop_suppl.to_owned());
                    continue;
                }
            }
        }
        if propnames.contains(&raw_header) {
            col_to_propname.insert_or_append(pos, raw_header.to_owned());
        } else {
            let lowered = raw_header.to_lowercase();
            println!("lowered: {}", lowered);
            if id_label.contains(&lowered.as_str()) {
                match lowered.as_str() {
                    "id" => {
                        col_to_id_label.insert_or_append(pos, Header::ID);
                    }
                    "label" => {
                        col_to_id_label.insert_or_append(pos, Header::Label);
                    }
                    _ => {panic!()}
                }
            }

        }
    }
    Ok((col_to_propname, col_to_prop_suppl, col_to_res_suppl, col_to_id_label))
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
fn compare_header_to_data_model(res_name: &String, dm_resources: &Vec<DMResource>, prop_names: &Vec<&String>, bitstream: Option<&Header>) -> Result<(), ExcelDataError> {
    let resource = match dm_resources.iter().find(|resource| resource.name.eq(res_name)) {
        None => { return Err(ExcelDataError::ParsingError(format!("not found resource with name '{}' in data-model", res_name))) }
        Some(dm_resource) => { dm_resource }
    };
    let propnames_lower: Vec<String> = prop_names.iter().map(|propname|propname.to_lowercase()).collect();
    let missing_propnames:Vec<_> = resource.properties.iter().map(|property|property.propname.to_lowercase()).filter(|propname| !propnames_lower.contains(propname)).collect();
    let missing_required_propnames = filter_required_propnames(missing_propnames, resource);

    if missing_required_propnames.len() != 0 {
        return Err(ExcelDataError::ParsingError(format!("not found all required propnames. Missing required propnames: {:?}. EXISTING HEADERS: {:?}.", missing_required_propnames, prop_names)))
    }
    /*
    if resource.super_field.ends_with("Representation") && bitstream.is_none() {

        return Err(ExcelDataError::ParsingError("Resource is a 'Representation' but not found bitstream-header in headers.".to_string()))
    }

     */
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

fn compare_header_to_res_prop(res_name: &String, res_prop_values: &Vec<&Header>) -> Result<(), ExcelDataError> {
    let should_have = [Header::ID, Header::Label];
    let doesnt_have = should_have.iter().filter(|header| !res_prop_values.contains(header)).collect::<Vec<_>>();
    if !doesnt_have.is_empty() {
        return Err(ExcelDataError::ParsingError(format!("For resource '{}'cannot find: {:?} in header, but it is mandatory.", res_name, doesnt_have)));
    }
    Ok(())
}

fn add_props_of_res(mut transient_data_header: &mut TransientDataHeader, row_nr_to_res_suppl: &&HashMap<usize, Vec<ResourceSupplement>>) -> Result<(), ExcelDataError> {
    for (pos, res_suppls) in row_nr_to_res_suppl.iter() {
        for res_suppl in res_suppls {
            match res_suppl.suppl_type {
                ResourceSupplType::IRI => {
                    transient_data_header.add_iri_pos(pos.to_owned())?;
                }
                ResourceSupplType::ARK => {
                    transient_data_header.add_ark_pos(pos.to_owned())?;
                }
                ResourceSupplType::Permissions => {
                    transient_data_header.add_permissions_pos(pos.to_owned())?;
                }
                ResourceSupplType::Bitstream => {
                    transient_data_header.add_bitstream_pos(pos.to_owned())?;
                }
                ResourceSupplType::BitstreamPermissions => {
                    transient_data_header.add_bitstream_permissions_pos(pos.to_owned())?;
                }
                ResourceSupplType::Authorship => {
                    transient_data_header.add_authorship_pos(pos.to_owned())?;
                }
                ResourceSupplType::License => {
                    transient_data_header.add_license_pos(pos.to_owned())?;
                }
                ResourceSupplType::CopyrightHolder => {
                    transient_data_header.add_copyright_holder_pos(pos.to_owned())?;
                }
            }
        }
    }
    Ok(())
}
fn add_propnames(transient_data_header: &mut TransientDataHeader, pos_to_propname: &&HashMap<usize, Vec<String>>, resource: &DMResource) -> Result<(), ExcelDataError> {
    let propnames_dm: Vec<_> = resource.properties.iter().map(|prop|&prop.propname).collect();
    _add_propnames(transient_data_header, pos_to_propname, &propnames_dm, resource.name.as_str())?;
    Ok(())
}

fn _add_propnames(transient_data_header: &mut TransientDataHeader, pos_to_propname: &&HashMap<usize, Vec<String>>, propnames_dm: &Vec<&String>, res_name: &str) -> Result<(), ExcelDataError>{
    for (pos, propnames) in pos_to_propname.iter() {
        for propname in propnames {
            if !propnames_dm.contains(&propname) {
                return Err(ExcelDataError::ParsingError(format!("Propname '{}' is a propname, but it is not part of resource '{}'", propname, res_name)));
            }
            transient_data_header.add_propname(propname.to_owned(), pos.to_owned())?;
        }
    }
    Ok(())
}

pub(crate) fn add_prop_suppl(transient_data_header: &mut TransientDataHeader, row_nr_to_prop_suppl: &&HashMap<usize, Vec<PropSupplement>>) {
    for (pos, prop_suppls) in row_nr_to_prop_suppl.iter() {
        for prop_suppl in prop_suppls {
            transient_data_header.add_prop_suppl(prop_suppl.to_owned(), pos.to_owned());
        }
    }
}
fn add_perm_comm_encod_of_properties(transient_data_header: &mut TransientDataHeader, pos_to_special_header: &HashMap<usize, Header>, pos_to_propname: &HashMap<usize, String>) -> Result<(), ExcelDataError>{
    /*
    let mut positions_special_prop: Vec<_> = pos_to_special_header.keys().collect::<Vec<_>>();
    positions_special_prop.sort();

    for (pos, prop_name) in pos_to_propname.iter() {
        let mut curr = pos.to_owned();
        //let mut transient_subheader = TransientSubheader::new();
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
            println!("values: subheader: {:?}", transient_subheader);
            let subheader = Subheader::new(transient_subheader.permissions, transient_subheader.encoding, transient_subheader.comment);
            transient_data_header.propname_to_subheader.insert(prop_name.to_string(), subheader);
        }
    }

     */
    Ok(())
}
