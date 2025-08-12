use std::collections::HashMap;
use std::path::PathBuf;
use regex::Regex;
use rust_fuzzy_search::fuzzy_compare;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::resource::DMResource;
use crate::create_hcl::errors::CreateHCLError;
use crate::create_hcl::hcl_resource::{HCLResource, WrapperHCLResource};

pub fn write_hcl(file_name_table_name_table_headers: Vec<(String, String, Vec<String>)>, datamodel: DataModel, dm_path: &PathBuf) -> Result<(), CreateHCLError> {
    let resources = hcl_resources(file_name_table_name_table_headers, &datamodel, dm_path)?;
    write_resources_to_hcl(resources)?;
    Ok(())
}
#[derive(Clone, Debug)]
pub enum NameType {
    FileName,
    TableName(String)
}

fn write_resources_to_hcl(hcl_resources: Vec<HCLResource>) -> Result<(), CreateHCLError>{
    for hcl_res in hcl_resources {
        println!("{:?}", hcl_res);
    }
    todo!()
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
        let mut table_name = match name_type {
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