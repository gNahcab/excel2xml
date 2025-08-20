use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::resource::DMResource;
use crate::create_hcl::errors::CreateHCLError;
use crate::create_hcl::supplement_hcl::{find_attached_type, to_supplement_type, AttachedToHeader, SupplementHCL, SupplementType};
use crate::create_hcl::write_hcl::clean_then_score;

static LABEL_ID: [&str; 2] = ["label", "id"];
static SUPPLEMENTS_HEADERS: [&str; 8] = ["permissions", "encoding", "authorship", "licenses", "bitstream_permissions", "copyright_holder", "ark", "iri"];
#[derive(Debug)]
pub struct HCLResource {
    pub shortcode: String,
    pub datamodel_path: String,
    pub resource_name: String,
    pub header_id_label: HashMap<String, String>,
    pub header_assignments: HashMap<String, String>,
    pub header_supplements: HashMap<String, SupplementHCL>,
    pub xlsx_path: String,
    pub sheet_nr: usize,
}

impl HCLResource {
    fn new(transient: TransientHCLResource) -> HCLResource {
        Self{
            shortcode: transient.shortcode.unwrap(),
            datamodel_path: transient.datamodel_path.unwrap(),
            resource_name: transient.resource_name.unwrap(),
            header_id_label: transient.header_id_label,
            header_assignments: transient.header_assignments,
            header_supplements: transient.header_supplements,
            xlsx_path: transient.xlsx_path.unwrap(),
            sheet_nr: transient.sheet_nr.unwrap(),
        }
    }
}

struct TransientHCLResource {
    shortcode: Option<String>,
    datamodel_path: Option<String>,
    resource_name: Option<String>,
    header_id_label: HashMap<String, String>,
    header_assignments: HashMap<String, String>,
    header_supplements: HashMap<String, SupplementHCL>,
    xlsx_path: Option<String>,
    table_name: Option<String>,
    sheet_nr: Option<usize> 
}


impl TransientHCLResource {
    pub(crate) fn add_table_name(&mut self, table_name: String) {
        self.table_name = Some(table_name)
    }
    pub(crate) fn add_header_id_label(&mut self, header_to_id_label: HashMap<String, String>) {
        self.header_id_label = header_to_id_label;
    }
    pub(crate) fn add_resource_name(&mut self, resource_name: String) {
        self.resource_name = Some(resource_name);
    }
    pub(crate) fn add_header_supplements(&mut self, header_supplements: HashMap<String, SupplementHCL>) {
        self.header_supplements = header_supplements;
    }
    pub(crate) fn add_header_assignments(&mut self, header_assignments: HashMap<String, String>) {
        self.header_assignments = header_assignments;
    }
    fn new(shortcode: String, dm_path: String, xlsx_path: String) -> Self {
        Self{
            shortcode: Some(shortcode),
            xlsx_path: Some(xlsx_path),
            datamodel_path: Some(dm_path),
            resource_name: None,
            header_id_label: Default::default(),
            header_assignments: Default::default(),
            header_supplements: Default::default(),
            table_name: None,
            sheet_nr: Some(1),
        }
    }

}
pub struct WrapperHCLResource(pub(crate) DMResource);

impl WrapperHCLResource {
    pub fn to_hcl_resource(self, data_model: &&DataModel, dm_path: &PathBuf, headers: &Vec<String>, xlsx_file: &String, table_name: Option<String>) -> Result<HCLResource, CreateHCLError> {
        let mut transient = TransientHCLResource::new(data_model.shortcode.to_owned(), dm_path.to_str().unwrap().to_string(), xlsx_file.to_owned());
        if table_name.is_some() {
            transient.add_table_name(table_name.unwrap().to_string());
        }
        transient.add_resource_name(self.0.name.to_owned());
        let (header_id_label, header_propname, header_supplements) = headers_to_id_label_propname_supplements(headers, &self.0)?;
        transient.add_header_assignments(header_propname);
        transient.add_header_supplements(header_supplements);
        transient.add_header_id_label(header_id_label);

        Ok(HCLResource::new(transient))
    }
}



fn headers_to_id_label_propname_supplements(headers: &Vec<String>, resource: &DMResource) -> Result<(HashMap<String, String>, HashMap<String, String>, HashMap<String, SupplementHCL>), CreateHCLError> {
    let mut already_visited: HashSet<usize> = HashSet::new();
    let pos_header_to_propname = match_header_to_names(headers, &resource.properties.iter().map(|prop|prop.propname.as_str()).collect(), &already_visited);
    already_visited.extend(pos_header_to_propname.keys().into_iter());
    let pos_header_to_id_label = match_header_to_names(headers, &LABEL_ID.to_vec().iter().map(|value|value.to_owned()).collect(), &already_visited);
    already_visited.extend(pos_header_to_id_label.keys().into_iter());
    let pos_header_to_supplement = match_header_to_names(headers, &SUPPLEMENTS_HEADERS.to_vec().iter().map(|value|value.to_owned()).collect(), &already_visited);
    already_visited.extend(pos_header_to_supplement.keys().into_iter());
    let header_to_supplement = map_headers_to_hcl_supplements(headers, &pos_header_to_propname, &pos_header_to_supplement, &pos_header_to_id_label)?;

    let header_to_propname: HashMap<String, String> = pos_header_to_propname.iter().map(|(pos, supplement)| (headers.get(pos.to_owned()).unwrap().to_owned(), supplement.to_owned())).collect();
    let header_to_id_label: HashMap<String, String> = pos_header_to_id_label.iter().map(|(pos, header_id)| (headers.get(pos.to_owned()).unwrap().to_owned(), header_id.to_owned())).collect();
    Ok((header_to_id_label, header_to_propname, header_to_supplement))
}

fn map_headers_to_hcl_supplements(headers: &Vec<String>, pos_header_to_propname: &HashMap<usize, String>, pos_header_to_supplement: &HashMap<usize, String>, pos_header_to_id_label: &HashMap<usize, String>,) -> Result<HashMap<String, SupplementHCL>, CreateHCLError>{
    let mut pos_header_to_hcl_supplement: HashMap<usize, SupplementHCL> = HashMap::new();
    for (pos_header, supplement) in pos_header_to_supplement {
        let supplement_hcl = map_header_tohcl_supplement(supplement, pos_header, headers, pos_header_to_propname, pos_header_to_id_label, &pos_header_to_hcl_supplement)?;
        pos_header_to_hcl_supplement.insert(pos_header.to_owned(), supplement_hcl);
    }
    Ok(pos_header_to_hcl_supplement.iter().map(|(pos_header,supplement_hcl)| (headers.get(pos_header.to_owned()).unwrap().to_owned(), supplement_hcl.to_owned())).collect())
}

fn map_header_tohcl_supplement(supplement: &String, pos_header: &usize, headers: &Vec<String>,
                               pos_header_to_propname: &HashMap<usize, String>,
                               pos_header_to_id_label: &HashMap<usize, String>,
                               pos_header_to_hcl_supplement: &HashMap<usize, SupplementHCL>)
    -> Result<SupplementHCL, CreateHCLError>{
    let supplement_type = to_supplement_type(supplement)?;
    // rules:
    // supplements that are attached to
    // - propnames  or bitstream are directly following
    let attached_header = if pos_header == &0usize {
        // first pos -> meaning: not attached to propname or bitstream
        AttachedToHeader::Resource
    } else if pos_header == &headers.len() {
        // last pos -> maybe attached to propname (matters if label/id comes before or not)
        find_attached_type(pos_header, headers, pos_header_to_propname, pos_header_to_id_label, &pos_header_to_hcl_supplement)
    } else {
        //betwixt -> maybe attached to propname (matters if label/id comes before or not)
        find_attached_type(pos_header, headers, pos_header_to_propname, pos_header_to_id_label, &pos_header_to_hcl_supplement)
    };
    Ok(SupplementHCL::new(supplement_type, attached_header))
}

fn match_header_to_names<'a>(headers: &Vec<String>, names: &'a Vec<&'a str>, already_visited: &HashSet<usize>) -> HashMap<usize, String> {
    // match all headers to names, take the header with the highest score (if that's more than 0)
    // if curr_pos was already matched, continue
    // return the header with the highest score as key and name as value and the positions in the header-vec that couldn't be mapped to a name

    let mut propname_position_score: HashMap<&'a str, Vec<(usize, f32)>> = HashMap::new();
    for (curr_pos, header) in headers.iter().enumerate() {
        if already_visited.contains(&curr_pos) {
            continue;
        }
        // match propnames
        for name in names.iter() {
            let score = clean_then_score(&name.to_string(), header);
            if !propname_position_score.contains_key(name) {
                propname_position_score.insert(name, vec![]);
            }
            propname_position_score.get_mut(name).unwrap().push((curr_pos, score));
        }
    }
    let mut header_pos_to_propname: HashMap<usize, String> = HashMap::new();
    for (propname, mut position_score) in propname_position_score.iter_mut() {
        position_score.sort_by(|a, b|a.1.total_cmp(&b.1));
        let last = position_score.last();
        if last.is_some() {
            let (pos, score) = last.unwrap();
            if score > &0f32 {
                header_pos_to_propname.insert(pos.to_owned(), propname.to_owned().to_owned());
            }
        }
    }
    header_pos_to_propname
}

fn not_matched(position_to_name: &Vec<usize>, last: usize) -> Vec<usize> {
    let mut not_matched = vec![];
    for i in 0..last {
        if !position_to_name.contains(&i) {
            not_matched.push(i);
        }
    }
    not_matched
}


