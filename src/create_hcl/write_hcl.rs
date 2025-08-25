use std::collections::HashMap;
use std::fmt::{format, Debug};
use std::fs::File;
use std::path::PathBuf;
use hcl::{attribute, Block, BlockBuilder, Identifier};
use hcl::edit::structure::Attribute;
use regex::Regex;
use rust_fuzzy_search::fuzzy_compare;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::resource::DMResource;
use crate::create_hcl::errors::CreateHCLError;
use crate::create_hcl::hcl_resource::{HCLResource, WrapperHCLResource};
use crate::create_hcl::supplement_hcl::{AttachedToHeader, SupplementHCL};

pub fn write_hcl(file_name_table_name_table_headers: Vec<(String, String, Vec<String>)>, datamodel: DataModel, dm_path: &PathBuf, folder_path: &PathBuf) -> Result<(), CreateHCLError> {
    let resources = hcl_resources(file_name_table_name_table_headers, &datamodel, dm_path)?;
    let non_existing_path = non_existing_path(datamodel.shortname);
    write_resources_to_hcl(resources, non_existing_path,folder_path, dm_path)?;
    Ok(())
}

fn non_existing_path(shortname: String) -> PathBuf {
    let mut name = format!("{}.hcl", shortname);
    let mut path_buf = PathBuf::from(name);
    let mut nr = 1;
    loop {
        if !path_buf.try_exists().unwrap() {
            break;
        }
        name = format!("{}_{}.hcl", shortname, nr);
        path_buf = PathBuf::from(name);
        nr +=1;
    }
    path_buf
}

#[derive(Clone, Debug)]
pub enum NameType {
    FileName,
    TableName(String)
}

fn write_resources_to_hcl(hcl_resources: Vec<HCLResource>, new_path: PathBuf, resources_folder_path: &PathBuf, dm_path: &PathBuf) -> Result<(), CreateHCLError>{
    let resources_folder_path = rel_path(resources_folder_path, 0)?;
    let dm_path = rel_path(dm_path, 1)?;
    let mut buffer = File::create(new_path)?;
    add_attribute(&mut buffer, "resources_folder_path", resources_folder_path.as_str())?;
    add_attribute(&mut buffer, "set_permissions", "TO_ADD")?;
    add_attribute(&mut buffer, "datamodel_path", dm_path.as_str())?;
    add_attribute(&mut buffer, "separator", "TO_ADD")?;

    for hcl_res in hcl_resources {
        let attribute = Attribute::new(Identifier::new("resource")?, hcl_res.resource_name.to_owned());
        let block_builder = BlockBuilder::new("xlsx")
            .add_block(
                sheet_block(&hcl_res, attribute)?
            )
            .add_label(hcl_res.xlsx_path);
        let block = block_builder.build();
        hcl::format::to_writer(&buffer, &block)?;
    }
    Ok(())
}

fn rel_path (path: &PathBuf, up: usize) -> Result<String, CreateHCLError> {
    // return relative path
    // up: number of ancestors to traverse in the path upwards
    // if there are less ancestors to traverse upwards: stop and return whole path
    let up = up + 1;
    let path = match path.to_str() {
        None => {
            return Err(CreateHCLError::InputError(format!("Cannot convert '{:?}' to str", path)))
        }
        Some(path) => {
            path
        }
    };
    let positions: Vec<usize> = path.match_indices("/").into_iter().map(|(pos, value)|pos).collect::<Vec<_>>().iter().rev().take(up).map(|pos|pos.to_owned()).collect::<Vec<usize>>();
    if positions.len() < up {
        return Ok(path.to_string())
    }
    let last = positions.last().unwrap() + 1;
    Ok(path[last..].to_string())
}

fn add_attribute(mut buffer: &mut File, key: &str, value: &str) -> Result<(), CreateHCLError> {
    let attribute = hcl::Attribute::new(Identifier::new(key)?, value);
    hcl::format::to_writer(&mut buffer, &attribute)?;
    Ok(())
}

fn sheet_block(hcl_res: &HCLResource, attribute: Attribute) -> Result<Block, CreateHCLError> {
    let block_builder = BlockBuilder::new("sheet")
        .add_label(format!("{}", &hcl_res.sheet_nr))
        .add_attribute(attribute)
        .add_blocks(assignments_block(&hcl_res.header_assignments, &hcl_res.header_id_label))
        .add_blocks(supplements_block(&hcl_res.header_supplements))
        .add_blocks(transforms_block(&hcl_res.transforms));
    Ok(block_builder.build())
}

fn transforms_block(transforms: &Vec<String>) -> Result<Block, CreateHCLError> {
    todo!()
}

fn block_builder(identifier: &str, ) -> Result<Block, CreateHCLError> {
    // todo abstract function
    //Vec<A>, for add to block
    //block_builder("supplements", supplements(&hcl_res.header_supplements)?)?;
    /*
    let block_builder = BlockBuilder::new(identifier).
        add_blocks(
        );
    Ok(block_builder.build())

     */
    todo!()
}
fn supplements_block(header_supplements: &HashMap<String, SupplementHCL>) -> Result<Block, CreateHCLError> {
    let block_builder = BlockBuilder::new("supplements").
        add_blocks(
            supplements(header_supplements)?
        );
    Ok(block_builder.build())
}

fn supplements (header_supplements: &HashMap<String, SupplementHCL>) -> Result<Vec<Block>, CreateHCLError> {
    let mut block_supplements = vec![];
    let mut block_name_to_attributes: HashMap<String, Vec<Attribute>> = HashMap::new();
    for (header, supplement) in header_supplements {
        let supplement_id = Identifier::new(
            format!("{:?}", supplement.supplement_type).to_lowercase())?;
        let identifier = match &supplement.attached_to_header {
            AttachedToHeader::Resource => {
                "resource".to_string()
            }
            AttachedToHeader::Propname(propname) => {propname.to_string()}
        };
        let attribute = Attribute::new(supplement_id,header.to_owned());
        if !block_name_to_attributes.contains_key(&identifier) {
            block_name_to_attributes.insert(identifier.to_string(), vec![]);
        }
        block_name_to_attributes.get_mut(&identifier).unwrap().push(attribute);
    }
    for (identifier, attributes) in block_name_to_attributes.iter() {
        let resource_block = BlockBuilder::new(identifier.as_str()).add_attributes(attributes.to_vec()).build();
        block_supplements.push(resource_block);
    }
    Ok(block_supplements)
}

fn assignments_block(header_assignments: &HashMap<String, String>, header_id_label: &HashMap<String, String>) -> Result<Block, CreateHCLError>{
    let block_builder = BlockBuilder::new("assignments").
        add_attributes(
            assignments(header_assignments, header_id_label)?
        );
    Ok(block_builder.build())
}
fn assignments(assignments: &HashMap<String, String>, header_id_label: &HashMap<String, String>) -> Result<Vec<Attribute>, CreateHCLError> {
    let mut attribute_assignments = vec![];
    for (header, assign) in assignments {
        let attribute = Attribute::new(Identifier::new(assign)?, header.to_owned());
        attribute_assignments.push(attribute);
    }
    for (header, id_label) in header_id_label {
        let attribute = Attribute::new(Identifier::new(id_label)?, header.to_owned());
        attribute_assignments.push(attribute);
    }
    Ok(attribute_assignments)
}

fn clean(value: &String) -> String {
    // replace any non-alphanumeric pieces with whitespace
    let re = Regex::new(r"[^a-zA-Z0-9\s]+").unwrap();
    let new = re.replace_all(value.as_str(), " ");
    new.to_string()
}
fn remove_ending(file_name: &&String) -> String {
    match file_name.rfind(".") {
        None => {
            file_name.to_string()
        }
        Some(pos) => {
            file_name[..pos].to_string()
        }
    }
}
fn hcl_resources(file_name_table_name_table_headers: Vec<(String, String, Vec<String>)>, datamodel: &DataModel, dm_path: &PathBuf) -> Result<Vec<HCLResource>, CreateHCLError>{
    let dm_res_with_infos = map_dm_res_to_infos(&file_name_table_name_table_headers, &datamodel);
    let mut hcl_resources = vec![];
    for (dm_res, name_type, file_name, headers) in dm_res_with_infos {
        let table_name = match name_type {
            NameType::FileName => {
                None
            }
            NameType::TableName(table_name) => {
                Some(table_name)
            }
        };
        println!("new hcl-resource based on file-name: {} and table-name: {:?}", file_name, table_name);
        let hcl_resource: HCLResource = WrapperHCLResource(dm_res.to_owned()).to_hcl_resource(&datamodel, dm_path, headers, file_name, table_name)?;
        hcl_resources.push(hcl_resource);
    }
    Ok(hcl_resources)
}

pub fn clean_then_score(value: &String, comparison: &String) -> f32 {
    let value = clean(value);
    let comparison = clean(comparison);
    fuzzy_compare(value.as_str(), comparison.as_str())
}
fn collect_scores<'a>(value: &String, candidates: Vec<&'a String>) -> Vec<(&'a String, f32)> {
    let mut candidate_score: Vec<(&String, f32)> = vec![];
    for candidate in candidates {
        let score = clean_then_score(value, candidate);
        candidate_score.push((candidate,score));
    }
    candidate_score
}

fn map_dm_res_to_infos<'a>(file_name_table_name_table_headers: &'a Vec<(String, String, Vec<String>)>, datamodel: &&'a DataModel) -> Vec<(&'a DMResource, NameType, &'a String, &'a Vec<String>)> {
    // maps all res_names to the file- and table-names with their respective scores (if the score > 0), then chooses the name with the highest score; if there are names mapping two the same res-name, the higher score is taken
    let mut res_name_to_dm_res: HashMap<&String, &'a DMResource> = datamodel.resources.iter().map(|dm_res|(&dm_res.name, dm_res)).collect();
    let mut res_name_with_name_type_scores:HashMap<&String, Vec<(NameType, f32, &'a String, &'a Vec<String>)>> = HashMap::new();
    for res_name in res_name_to_dm_res.keys() {
        res_name_with_name_type_scores.insert(res_name.to_owned(), vec![]);
        for (file_name, table_name, headers) in file_name_table_name_table_headers.iter() {
            let score_t_n = clean_then_score(table_name, res_name);
            let score_f_n = clean_then_score(file_name, res_name);
            if score_f_n > 0f32 && score_f_n >= score_t_n  {
                if let Some(val) = res_name_with_name_type_scores.get_mut(res_name) { val.push((NameType::FileName, score_f_n, file_name,  headers)) };
            } else if score_t_n > 0f32 {
                if let Some(val) = res_name_with_name_type_scores.get_mut(res_name) { val.push((NameType::TableName(table_name.to_owned()),  score_t_n, file_name, headers)) };
            }
        }
    }
    let mut dm_res_with_infos: Vec<(&'a DMResource, NameType, &'a String, &'a Vec<String>)> = vec![];
    for (res_name, mut name_types_scores  ) in res_name_with_name_type_scores.iter_mut() {
        name_types_scores.sort_by(|a, b|a.1.total_cmp(&b.1));
        match  name_types_scores.first() {
            None => {
                //ignore
            }
            Some((name_type, score, file_name, headers)) => {
                let dm_res = res_name_to_dm_res.get(res_name).unwrap();
                dm_res_with_infos.push((dm_res, name_type.to_owned(), file_name, headers));
            }
        }
    }
    dm_res_with_infos
    }