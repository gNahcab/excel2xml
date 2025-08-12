use std::collections::HashMap;
use std::path::PathBuf;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::property::Property;
use crate::parse_dm::domain::resource::DMResource;
use crate::create_hcl::errors::CreateHCLError;
use crate::create_hcl::supplement_hcl::{find_attached_type, to_supplement_type, AttachedHeader, SupplementHCL, SupplementType};
use crate::create_hcl::write_hcl::clean_then_score;
use crate::parse_dm::domain::res_property::ResProperty;

static LABEL_ID: [&str; 2] = ["label", "id"];
static SUPPLEMENTS_HEADERS: [&str; 8] = ["permissions", "encoding", "authorship", "licenses", "bitstream_permissions", "copyright_holder", "ark", "iri"];
#[derive(Debug)]
pub struct HCLResource {
    shortcode: String,
    datamodel_path: String,
    resource_name: String,
    assignments: HashMap<String, String>,
    supplements: HashMap<String, SupplementType>,
    xlsx_path: String
}

impl HCLResource {
    fn new(transient: TransientHCLResource) -> HCLResource {
        Self{
            shortcode: transient.shortcode.unwrap(),
            datamodel_path: transient.datamodel_path.unwrap(),
            resource_name: transient.resource_name.unwrap(),
            assignments: transient.assignments,
            supplements: transient.supplements,
            xlsx_path: transient.xlsx_path.unwrap(),
        }
    }
}

struct TransientHCLResource {
    shortcode: Option<String>,
    datamodel_path: Option<String>,
    resource_name: Option<String>,
    assignments: HashMap<String, String>,
    supplements: HashMap<String, SupplementType>,
    xlsx_path: Option<String>,
    table_name: Option<String>
}

impl TransientHCLResource {
    pub(crate) fn add_table_name(&mut self, table_name: String) {
        self.table_name = Some(table_name)
    }
}

impl TransientHCLResource {
    pub(crate) fn add_resource_name(&mut self, resource_name: String) {
        self.resource_name = Some(resource_name);
    }
    pub(crate) fn add_supplements(&mut self, supplements: HashMap<String, SupplementType>) {
        self.supplements = supplements;
    }
    pub(crate) fn add_assignments(&mut self, assignments: HashMap<String, String>) {
        self.assignments = assignments;
    }
    fn new(shortcode: String, dm_path: String, xlsx_path: String) -> Self {
        Self{
            shortcode: Some(shortcode),
            xlsx_path: Some(xlsx_path),
            datamodel_path: Some(dm_path),
            resource_name: None,
            assignments: Default::default(),
            supplements: Default::default(),
            table_name: None,
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
        let (assignments, supplements) = headers_to_assignments_supplements(headers, &self.0, &data_model.properties)?;
        transient.add_assignments(assignments);
        transient.add_supplements(supplements);

        Ok(HCLResource::new(transient))
    }
}



fn headers_to_assignments_supplements(headers: &Vec<String>, resource: &DMResource, properties: &Vec<Property>) -> Result<(HashMap<String, String>, HashMap<String, SupplementType>), CreateHCLError> {
    let pos_to_score_propname: HashMap<usize, (usize, String)> = HashMap::new();
    let mut pos_to_header: HashMap<usize, String> = HashMap::new();
    let mut no_match_found = false;
    let mut already_visited = vec![];
    let pos_header_to_propname = match_header_to_names(headers, &resource.properties.iter().map(|prop|prop.propname.as_str()).collect(), &already_visited);
    already_visited.append(&mut pos_header_to_propname.keys().into_iter().map(|key|key.to_owned()).collect());
    let pos_header_to_id_label = match_header_to_names(headers, &LABEL_ID.to_vec().iter().map(|value|value.to_owned()).collect(), &already_visited);
    already_visited.append(&mut pos_header_to_propname.keys().into_iter().map(|key|key.to_owned()).collect());
    let pos_header_to_supplement = match_header_to_names(headers, &SUPPLEMENTS_HEADERS.to_vec().iter().map(|value|value.to_owned()).collect(), &already_visited);
    already_visited.append(&mut pos_header_to_propname.keys().into_iter().map(|key|key.to_owned()).collect());
    let header_to_supplement = verify_header_to_supplement(headers,&resource.properties, &pos_header_to_propname, already_visited, &pos_header_to_supplement, &pos_header_to_id_label)?;

    let header_to_propname: HashMap<String, String> = pos_header_to_supplement.iter().map(|(pos, supplement)| (headers.get(pos.to_owned()).unwrap().to_owned(), supplement.to_owned())).collect();
    let header_to_id_label: HashMap<String, String> = pos_header_to_id_label.iter().map(|(pos, header_id)| (headers.get(pos.to_owned()).unwrap().to_owned(), header_id.to_owned())).collect();
    todo!()
}

fn verify_header_to_supplement(headers: &Vec<String>, res_properties: &Vec<ResProperty>, pos_header_to_propname: &HashMap<usize, String>, already_visited: Vec<usize>, pos_header_to_supplement: &HashMap<usize, String>, pos_header_to_id_label: &HashMap<usize, String>,) -> Result<HashMap<String, String>, CreateHCLError>{
    let mut pos_header_to_hcl_supplement: HashMap<usize, SupplementHCL> = HashMap::new();
    for (pos_header, supplement) in pos_header_to_supplement {
        let supplement_type = to_supplement_type(supplement)?;
        // rules:
        // supplements that are attached to
        // - propnames  or bitstream are directly following
        let attached_header = if pos_header == &0usize {
            // first pos -> meaning: not attached to propname or bitstream
            AttachedHeader::Resource
        } else if pos_header == &headers.len() {
            // last pos -> maybe attached to propname (matters if label/id comes before or not)
            find_attached_type(pos_header, headers, pos_header_to_propname, pos_header_to_id_label, &pos_header_to_hcl_supplement)
        } else {
            //betwixt -> maybe attached to propname (matters if label/id comes before or not)
            find_attached_type(pos_header, headers, pos_header_to_propname, pos_header_to_id_label, &pos_header_to_hcl_supplement)
        };
        let supplement_hcl = SupplementHCL::new(supplement_type, attached_header);
        pos_header_to_hcl_supplement.insert(pos_header.to_owned(), supplement_hcl);
    }
    todo!();
}


fn match_header_to_names<'a>(headers: &Vec<String>, names: &'a Vec<&'a str>, already_visited: &Vec<usize>) -> HashMap<usize, String> {
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


