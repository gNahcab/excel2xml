use std::collections::HashMap;
use std::path::PathBuf;
use nucleo::{Config, Matcher, Utf32Str};
use regex::Regex;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::property::Property;
use crate::parse_dm::domain::resource::DMResource;
use crate::write_hcl::errors::WriteHCLError;
use crate::write_hcl::supplement_hcl::{SupplementHCL, SupplementType};

pub struct HCLResource {
    shortcode: String,
    datamodel_path: String,
    resource_name: String,
    assignments: HashMap<String, String>,
    supplements: HashMap<String, SupplementHCL>,
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
    supplements: HashMap<String, SupplementHCL>,
    xlsx_path: Option<String>
}

impl TransientHCLResource {
    pub(crate) fn add_resource_name(&mut self, resource_name: String) {
        self.resource_name = Some(resource_name);
    }
    pub(crate) fn add_supplements(&mut self, supplements: HashMap<String, SupplementHCL>) {
        self.supplements = supplements;
    }
    pub(crate) fn add_assignments(&mut self, assignments: HashMap<String, String>) {
        self.assignments = assignments;
    }
    fn new(shortcode: String, xlsx_path: String, dm_path: String) -> Self {
        Self{
            shortcode: Some(shortcode),
            xlsx_path: Some(xlsx_path),
            datamodel_path: Some(dm_path),
            resource_name: None,
            assignments: Default::default(),
            supplements: Default::default(),
        }
    }

}
pub struct WrapperHCLResource(pub(crate) (String, String, Vec<String>));

impl WrapperHCLResource {
    pub fn to_hcl_resource(self, data_model: &DataModel, dm_path: &PathBuf) -> Result<HCLResource, WriteHCLError> {
        let (xlsx_file, table_name, headers) = self.0;
        let mut transient = TransientHCLResource::new(data_model.shortcode.to_owned(), xlsx_file.to_owned(), dm_path.to_str().unwrap().to_string());
        let resource = find_resource(xlsx_file, table_name, &data_model.resources)?;
        transient.add_resource_name(resource.name.to_owned());
        let (assignments, supplements) = headers_to_assignments_supplements(headers, resource, &data_model.properties)?;
        transient.add_assignments(assignments);
        transient.add_supplements(supplements);

        Ok(HCLResource::new(transient))

    }
}

fn find_resource(xlsx_file_name: String, table_name: String, resources: &Vec<DMResource>) -> Result<&DMResource, WriteHCLError> {
    let xlsx_file_name = remove_ending(xlsx_file_name);
    for resource in resources {
        if is_match(xlsx_file_name.to_owned(), resource.name.to_owned()) {
            return Ok(resource)
        }
        if is_match(table_name.to_owned(), resource.name.to_owned()) {
            return Ok(resource)
        }
    }
    Err(WriteHCLError::NotFoundError(format!("Cannot identify with the file-name '{}' or table-name '{}' the resource associated. Resource-names: {:?}", &xlsx_file_name, &table_name, resources.iter().map(|resource|&resource.name).collect::<Vec<_>>())))
}

fn remove_ending(file_name: String) -> String {
    match file_name.rfind(".") {
        None => {
             file_name
        }
        Some(pos) => {
            file_name[..pos].to_string()
        }
    }
}

fn is_match(haystack: String, needle: String) -> bool {
    let haystack = clean(haystack);
    let needle = clean(needle);
    let mut buffer1 = vec![];
    let mut matcher = Matcher::new(Config::DEFAULT);
    let haystack = Utf32Str::new(haystack.as_str(), &mut buffer1);
    let mut buffer2 = vec![];
    let needle = Utf32Str::new(needle.as_str(), &mut buffer2);
    let result = matcher.fuzzy_match(haystack, needle);
    if result.is_some() {
        return true
    }
    let result = matcher.fuzzy_match(needle, haystack);
    result.is_some()
}

fn clean(value: String) -> String {
    // replace any non-alphanumeric pieces with whitespace
    let re = Regex::new(r"[^a-zA-Z0-9\s]+").unwrap();
    let new = re.replace_all(value.as_str(), " ");
    new.to_string()
}

fn headers_to_assignments_supplements(headers: Vec<String>, resource: &DMResource, properties: &Vec<Property>) -> Result<(HashMap<String, String>, HashMap<String, SupplementHCL>), WriteHCLError> {
    let mut assignments:HashMap<String, String> = HashMap::new();
    let assignment_headers = ["label", "id"];
    let supplement_headers = ["bitstream","permission", "permissions", "bitstream_permissions", "bitstream permission", "bitstream permissions", "encoding", "comment", "authorship", "license", "licenses", "copyright_holder", "copyright holder"];
    let mut pos_to_header: HashMap<usize, String> = HashMap::new();
    let mut supplements: HashMap<String, SupplementHCL> = HashMap::new();
    let mut no_match_found = false;
    for (curr_pos, header) in headers.iter().enumerate() {
        // match propnames
        for propname in resource.properties.iter() {

            no_match_found = true;
            if is_match(header.to_owned(), propname.propname.to_owned()) {
                pos_to_header.insert(curr_pos, header.to_string());
                assignments.insert(header.to_owned(), propname.propname.to_owned());
                no_match_found = false;
                break;
            }
        }
        // match id, label (additional assignment-headers)
            if no_match_found {
                for assignment_h in assignment_headers {
                    let assignment_h = assignment_h.to_string();
                    if is_match(header.to_owned(), assignment_h.to_owned()) {
                        pos_to_header.insert(curr_pos, header.to_string());
                        assignments.insert(header.to_owned(), assignment_h);
                        no_match_found = false;
                        break;
                    }
                }
            }
        // match supplement headers
        if no_match_found {
            for supplement_h in supplement_headers {
                let supplement_h = supplement_h.to_string();
                if is_match(header.to_owned(), supplement_h.to_owned()) {
                    let assigned_to = find_assigned_to(&pos_to_header, curr_pos, &supplements, &assignments);
                    match supplement_h.as_str() {
                        "bitstream" => {
                            let supplement_hcl = SupplementHCL::new(SupplementType::Bitstream, header.to_owned());
                            supplements.insert(header.to_owned(), supplement_hcl);
                        }
                        "bitstream_permissions" => {
                            let supplement_hcl = SupplementHCL::new(SupplementType::BitstreamPermission, header.to_owned());
                            supplements.insert(header.to_owned(), supplement_hcl);
                        }
                        "permissions"  => {
                            let supplement_hcl = SupplementHCL::new(SupplementType::Permissions(assigned_to.unwrap()), header.to_owned());
                            supplements.insert(header.to_owned(), supplement_hcl);
                        }
                        "comment" => {
                            let supplement_hcl = SupplementHCL::new(SupplementType::Comment(assigned_to.unwrap()), header.to_owned());
                            supplements.insert(header.to_owned(), supplement_hcl);
                        }
                        "encoding"  => {
                            let supplement_hcl = SupplementHCL::new(SupplementType::Encoding(assigned_to.unwrap()), header.to_owned());
                            supplements.insert(header.to_owned(), supplement_hcl);
                        }
                        "copyright_holder"| "copyright holder"  => {
                            let supplement_hcl = SupplementHCL::new(SupplementType::CopyrightHolder, header.to_owned());
                            supplements.insert(header.to_owned(), supplement_hcl);
                        }
                        "license"|"licenses"  => {
                            let supplement_hcl = SupplementHCL::new(SupplementType::License, header.to_owned());
                            supplements.insert(header.to_owned(), supplement_hcl);
                        }
                        "authorship"  => {
                            let supplement_hcl = SupplementHCL::new(SupplementType::Authorship, header.to_owned());
                            supplements.insert(header.to_owned(), supplement_hcl);
                        }
                        _ => {
                            panic!("Not added to supplements: {}", supplement_h)
                        }
                    }
                    pos_to_header.insert(curr_pos, header.to_string());
                    no_match_found = false;
                    break;
                }
            }
        }
    }
    return Ok((assignments, supplements))
}

fn find_assigned_to(pos_to_header: &HashMap<usize, String>, curr_pos: usize, supplements: &HashMap<String, SupplementHCL>, assignments: &HashMap<String, String>) -> Option<String> {
    if curr_pos <= 0 {
        return None
    }
    let before = curr_pos - 1;
    let header = pos_to_header.get(&before).unwrap();
    match supplements.get(header) {
        None => {}
        Some(supplement) => {
            return match &supplement.supplement_type {
                SupplementType::Authorship => {
                    Some("resource".to_string())
                }
                SupplementType::License => {
                    Some("resource".to_string())
                }
                SupplementType::CopyrightHolder => {
                    Some("resource".to_string())
                }
                SupplementType::Encoding(assigned_to) => {
                    Some(assigned_to.to_owned())
                }
                SupplementType::Comment(assigned_to) => {
                    Some(assigned_to.to_owned())
                }
                SupplementType::Bitstream => {
                    Some("resource".to_string())
                }
                SupplementType::BitstreamPermission => {
                    Some("resource".to_string())
                }
                SupplementType::Permissions(assigned_to) => {
                    Some(assigned_to.to_owned())
                }
            }
        }
    }
    match assignments.get(header) {
        None => {
        }
        Some(assignment) => {
            return Some(assignment.to_owned());
        }
    }
    panic!("This should never happen.")
}

