use std::collections::HashMap;
use std::fmt::Debug;
use clap::builder::TypedValueParser;
use crate::parse_dm::domain::dasch_list::{DaSCHList, ListNode};
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::label::Label;
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::header_value::HeaderValue;
use crate::parse_hcl::methods_domain::behavior_type::BehaviorType;
use crate::parse_hcl::methods_domain::combine_method::CombineMethod;
use crate::parse_hcl::methods_domain::create_method::{CreateMethod};
use crate::parse_hcl::methods_domain::date_pattern::DatePattern;
use crate::parse_hcl::methods_domain::date_type::DateType;
use crate::parse_hcl::methods_domain::integer_create::IntegerCreate;
use crate::parse_hcl::methods_domain::lower_upper_method::{LowerMethod, UpperMethod};
use crate::parse_hcl::methods_domain::permissions_create::PermissionsCreate;
use crate::parse_hcl::methods_domain::replace_label_name::ReplaceLabelNameMethod;
use crate::parse_hcl::methods_domain::replace_method::ReplaceMethod;
use crate::parse_hcl::methods_domain::step::{StepMethod};
use crate::parse_hcl::methods_domain::to_alter_method::AlterMethod;
use crate::parse_hcl::methods_domain::to_date_method::ToDateMethod;
use crate::parse_hcl::methods_domain::update_with_server_method::ReplaceWithIRI;
use crate::parse_xlsx::domain::data_col::{DataCol, TransientDataCol};
use crate::parse_xlsx::domain::data_domain::date_period::DatePeriodWrapper;

pub fn perform_identify(key_value_map: HashMap<String, String>, base_col: &Vec<String>, separator: &String) -> Vec<String> {
    // expand by separator to vec and then replace all instances, finally using join to return as String
    base_col.iter()
        .map(|maybe_key| maybe_key.split(separator))
        .map(|maybe_keys|maybe_keys.into_iter()
            .map(|key|match key_value_map.get(key.trim()) {
        None => {
            if !key.starts_with("http") && !key.trim().is_empty(){
                println!("NONE: {}", key);
            }
            key.to_string()}
        Some(replace) => {
            replace.to_owned()}
    }).collect::<Vec<String>>()).map(|values|values.join(separator)).collect()
}


pub fn perform_replace_label_name(replace_label_name_method: &ReplaceLabelNameMethod, col_nr_to_cols_expanded: &HashMap<usize, DataCol>, header_to_col_nr_expanded: &HashMap<String, usize>, data_model: &&DataModel, separator: &String) -> Result<DataCol, HCLDataError> {
    let header_number = find_header_number(&replace_label_name_method.input, col_nr_to_cols_expanded, header_to_col_nr_expanded)?;
    let col = &col_nr_to_cols_expanded.get(&header_number).unwrap();
    let labels_to_names = _labels_to_names(data_model, &replace_label_name_method.list_name)?;
    let new_column = _replace_label_name(col, labels_to_names, separator);
    Ok(DataCol::new(new_column, replace_label_name_method.output.to_owned()))
}

fn _labels_to_names(data_model: &&DataModel, list_name: &String) -> Result<HashMap<String, String>, HCLDataError> {
    let mut list = match data_model.lists.get(list_name) {
        None => {
            return Err(HCLDataError::InputError(format!("List with name '{}' not found in lists. Existing listnames: '{:?}'", list_name, data_model.lists.iter().map(|(name, _)|name).collect::<Vec<&String>>())));
        }
        Some(list) => {list}
    };
    let mut labels_names: Vec<(&Label, &String)> = vec![];
    _flatten_labels(&list.nodes, &mut labels_names);
    let label_to_name = labels_names.iter().map(|(label, name)| (label.label.to_owned(), name.to_owned().to_owned())).collect::<HashMap<String, String>>();
    Ok(label_to_name)
}

fn _flatten_labels<'node>(list_nodes: &'node Vec<ListNode>, labels_names: &mut Vec<(&'node Label, &'node String)>) {
    for node in list_nodes {
        for label in &node.labels {
            labels_names.push((label, &node.name));
        }
        _flatten_labels(&node.nodes, labels_names)
    }
}

fn _replace_label_name(data_col: &&DataCol, label_to_name: HashMap<String, String>, separator: &String) -> Vec<String> {
    data_col.col
        .iter()
        .map(|value| value.split(separator))
        .map(|values|values.into_iter()
            .map(|value| match label_to_name.get(value.trim()) {
                None => {
                    value.to_owned()
                }
                Some(new_value) => {
                    new_value.to_owned()}})
            .collect::<Vec<String>>())
        .map(|values| values.join(separator))
        .collect()
}
pub fn perform_replace_with_iri(replace_with_iri_method: &ReplaceWithIRI, col_nr_to_cols_expanded: &HashMap<usize, DataCol>, existing_header_to_col_nr: &HashMap<String, usize>, res_name_iri: &HashMap<String, HashMap<String, String>>,  separator: &String) -> Result<DataCol, HCLDataError> {
    let header_number = find_header_number(&replace_with_iri_method.input, col_nr_to_cols_expanded, existing_header_to_col_nr)?;
    let col = &col_nr_to_cols_expanded.get(&header_number).unwrap();
    let label_to_iri = match res_name_iri.get(replace_with_iri_method.resource.as_str()) {
        None => {return Err(HCLDataError::InputError(format!("Resource-name '{}' does not exist in res-name-to-label-iri. Existing names are: '{:?}'.", replace_with_iri_method.resource, res_name_iri.keys())))}
        Some(label_to_iri) => {label_to_iri}
    };
    let new_column = _replace_with_iri(col, label_to_iri, separator);
    Ok(DataCol::new(new_column, replace_with_iri_method.output.to_owned()))
}

fn _replace_with_iri(data_col: &&DataCol, label_to_iri: &HashMap<String, String>, separator: &String) -> Vec<String> {
    data_col.col
        .iter()
        .map(|value| value.split(separator))
        .map(|values|values.into_iter()
            .map(|label|  match label_to_iri.get(label.trim()) {
                None => { label.to_owned()}
                Some(iri) => { iri.to_owned()}})
            .collect::<Vec<String>>())
        .map(|values|values.join(separator))
        .collect()
}

pub fn perform_replace(replace_method: &ReplaceMethod, col_nr_to_cols_expanded: &HashMap<usize, DataCol>, existing_header_to_col_nr: &HashMap<String, usize>, separator: &String) -> Result<DataCol, HCLDataError> {
    let header_number = find_header_number(&replace_method.input, col_nr_to_cols_expanded, existing_header_to_col_nr)?;
    let col = &col_nr_to_cols_expanded.get(&header_number).unwrap();

    let new_column = _replace(col, &replace_method.new, &replace_method.old, &replace_method.behavior, separator);
    Ok(DataCol::new(new_column, replace_method.output.to_owned()))
}
pub fn perform_to_date(to_date_method: &ToDateMethod, col_nr_to_cols: &HashMap<usize, DataCol>, header_to_col_nr: &HashMap<String, usize>, separator: &String) -> Result<DataCol, HCLDataError> {
    let header_number = find_header_number(&to_date_method.input, col_nr_to_cols, header_to_col_nr)?;
    let col = &col_nr_to_cols.get(&header_number).unwrap();
    let new_col = _to_date(col, &to_date_method.date_patterns, &to_date_method.date_type, separator)?;
    Ok(DataCol::new(new_col, to_date_method.output.to_owned()))
}

fn _to_date(data_col: &&DataCol, date_patterns: &Vec<DatePattern>, date_type: &DateType, separator: &String) -> Result<Vec<String>, HCLDataError> {
    let mut new_col = vec![];
    for value in data_col.col.iter() {
        if value.is_empty() {
            new_col.push("".to_string());
        } else {
            let mut dates = vec![];
            for val in value.split(separator).into_iter() {
                let date_period = DatePeriodWrapper(val.to_owned()).to_date_period(date_patterns, date_type)?.to_date_period_string();
                dates.push(date_period);
            }
            let date_periods = dates.join(separator);
            new_col.push(date_periods);
        }
    }
    Ok(new_col)
}

pub fn perform_alter(alter_method: &AlterMethod, col_nr_to_cols: &HashMap<usize, DataCol>, header_to_col_nr: &HashMap<String, usize>) -> Result<DataCol, HCLDataError> {
    let header_number = find_header_number(&alter_method.input, col_nr_to_cols, header_to_col_nr)?;
    let data_col = &col_nr_to_cols.get(&header_number).unwrap();
    let mut new_col = vec![];
    for value in data_col.col.iter() {
        let mut new_value = "".to_string();
        if alter_method.prefix.is_some() {
            new_value += alter_method.prefix.as_ref().unwrap().as_str();
        }
        new_value += value.as_str();
        if alter_method.suffix.is_some() {
            new_value += alter_method.suffix.as_ref().unwrap().as_str();
        }
        new_col.push(new_value);
    }
    Ok(DataCol::new(new_col, alter_method.output.to_owned()))
}
pub fn perform_create(create_method: &CreateMethod, length: usize) -> DataCol {
    match create_method {
        CreateMethod::IntegerCreateMethod(int_create) => {
            perform_int_create(int_create, length)

        }
        CreateMethod::PermissionsCreateMethod(permissions_create) => {
            perform_permissions_create(permissions_create, length)

        }
    }
}

fn perform_permissions_create(permissions_create: &PermissionsCreate, length: usize) -> DataCol {
    let mut new_data_col = vec![];
    for _ in 0..length {
        new_data_col.push(permissions_create.permissions.to_string())
    }
    DataCol::new(new_data_col, permissions_create.output.to_owned())
}
fn perform_int_create(int_create: &IntegerCreate, length: usize) -> DataCol {
    let mut new_data_col = vec![];
    let mut curr = int_create.start;
    match int_create.step.step_method {
        StepMethod::Plus => {
            for _ in 0..length {
                curr = curr + int_create.step.step_rate;
                let mut value = "".to_string();
                if int_create.prefix.is_some() {
                    value.push_str(int_create.prefix.as_ref().unwrap())
                }
                value.push_str(curr.to_string().as_str());
                if int_create.suffix.is_some() {
                    value.push_str(int_create.prefix.as_ref().unwrap())
                }
                new_data_col.push(value);
            }
        }
        _ => {todo!("not added")}
        /*
        StepMethod::Multiplication => {
            for _ in 0..length {
                curr = curr * int_create.step.step_rate;
                let mut value = "".to_string();
                if int_create.prefix.is_some() {
                    value.push_str(int_create.prefix.as_ref().unwrap())
                }
                value.push_str(curr.to_string().as_str());
                if int_create.suffix.is_some() {
                    value.push_str(int_create.prefix.as_ref().unwrap())
                }
                new_data_col.push(value);
            }
        }

         */
    }
    DataCol::new(new_data_col, int_create.output.to_owned())
}

pub fn perform_combine(combine_method: &CombineMethod, col_nr_to_cols: &HashMap<usize, DataCol>, header_to_col_nr: &HashMap<String, usize>) -> Result<DataCol, HCLDataError> {
    todo!("add separator");
    let first_number = find_header_number(combine_method.input.get(0).unwrap(),col_nr_to_cols, header_to_col_nr)?;
    let second_number = find_header_number(combine_method.input.get(1).unwrap(), col_nr_to_cols, header_to_col_nr)?;
    let first_col = &col_nr_to_cols.get(&first_number).unwrap();
    let second_col = &col_nr_to_cols.get(&second_number).unwrap();
    let new_col = _combine(first_col, second_col, &combine_method.prefix, &combine_method.suffix, &combine_method.separator);
    Ok(DataCol::new(new_col, combine_method.output.to_owned()))
}

fn _combine(first_col: &&DataCol, second_col: &&DataCol, prefix: &Option<String>, suffix: &Option<String>, separator: &Option<String>) -> Vec<String> {
    todo!("add separator");
    let mut new_col = vec![];
    // it is assumed first_col and second_col have same length
    for i in 0..first_col.col.len() {
        let mut new_combined_value = "".to_string();
        if prefix.is_some() {
            new_combined_value += &prefix.as_ref().unwrap().as_str();
        }
        new_combined_value += first_col.col[i].as_str();
        if separator.is_some() {
            new_combined_value += &separator.as_ref().unwrap().as_str();
        }
        new_combined_value += second_col.col[i].as_str();
        if suffix.is_some() {
            new_combined_value += &suffix.as_ref().unwrap().as_str();
        }
        new_col.push(new_combined_value);
    }
    new_col
}

pub fn perform_upper(upper_method: &UpperMethod, col_nr_to_cols: &HashMap<usize, DataCol>, headers_to_col_nr: &HashMap<String, usize>) -> Result<DataCol, HCLDataError> {
    let header_number = find_header_number(&upper_method.input, col_nr_to_cols, headers_to_col_nr)?;
    let data_col = &col_nr_to_cols.get(&header_number).unwrap();
    let col = _upper(data_col);
    Ok(DataCol::new(col, upper_method.output.to_owned()))
}

fn _upper(data_col: &DataCol) -> Vec<String> {
    todo!("add separator");
    data_col.col.iter().map(|value| value.to_lowercase()).collect()
}

pub fn perform_lower(lower_method: &LowerMethod, col_nr_to_cols: &HashMap<usize, DataCol>, headers_to_col_nr: &HashMap<String, usize>) -> Result<DataCol, HCLDataError> {
    let header_number = find_header_number(&lower_method.input, col_nr_to_cols, headers_to_col_nr)?;
    let data_col = &col_nr_to_cols.get(&header_number).unwrap();
    let col = _lower(data_col);
    Ok(DataCol::new(col, lower_method.output.to_owned()))

}

fn _lower(data_col: &DataCol) -> Vec<String> {
    todo!("add separator");
    data_col.col.iter().map(|value| value.to_lowercase()).collect()
}
fn _replace(data_column: &&DataCol, new: &String, old: &String, behavior: &BehaviorType, separator: &String) -> Vec<String> {
     match behavior {
        BehaviorType::Lazy => {
            //let _ = data_column.column.iter().map(|value| transient_column.add_data(value.replacen(new, old, 1)));
            data_column.col.iter()
                .map(|value| value.split(separator))
                .map(|values|values.into_iter()
                    .map(|label| label.replacen(new, old, 1))
                    .collect::<Vec<_>>()).map(|new_values|new_values.join(separator)).collect::<Vec<_>>()
        }

        BehaviorType::Greedy => {
            //let _ = data_column.column.iter().map(|value| transient_column.add_data(value.replace(new, old)));
            data_column.col.iter()
                .map(|value| value.split(separator))
                .map(|values|values.into_iter()
                    .map(|label| label.replace(old, new))
                    .collect::<Vec<_>>()).map(|new_values|new_values.join(separator)).collect::<Vec<_>>()
        }
    }
}

fn find_header_number(input: &HeaderValue, col_nr_to_data_col: &HashMap<usize, DataCol>, header_to_col_nr: &HashMap<String, usize>) -> Result<usize, HCLDataError> {
    match &input {
        HeaderValue::Name(name) => {
            match header_to_col_nr.get(name) {
                None => {
                    return Err(HCLDataError::ParsingError(format!("Error during manipulation. Not found '{}' in headers: '{:?}'.", name, header_to_col_nr.iter().map(|(header, _)|header).collect::<Vec<&String>>())));}
                Some(number) => {Ok(number.to_owned())}
            }
        }
        HeaderValue::Number(number) => {
            let number= number.to_owned() as usize;
            if col_nr_to_data_col.contains_key(&number) {
                Ok(number.to_owned() as usize)
            } else {
                return Err(HCLDataError::ParsingError(format!("Error during manipulation. Not found header-number '{}' Probably out of bounds: '{:?}'.", number, header_to_col_nr.iter().map(|(header, _)|header).collect::<Vec<&String>>())));}
            }
        }
    }
#[cfg(test)]
mod test {
    use crate::parse_hcl::header_value::HeaderValue;
    use crate::parse_hcl::methods_domain::date_bricks::{DateBricks, DateInfo, DateName};
    use crate::parse_hcl::methods_domain::date_pattern::DatePattern;
    use crate::parse_hcl::methods_domain::date_type::DateType;
    use crate::parse_hcl::methods_domain::to_date_method::ToDateMethod;

    #[test]
    fn test_to_date() {
        let vec_1: Vec<String> = ["01.01.1991".to_string(), "3.2.400".to_string(), "2.January 1991".to_string()].to_vec();
        let date_method = ToDateMethod {
            output: "hasDate".to_string(),
            input: HeaderValue::Name("hasDateRaw".to_string()),
            date_type: DateType::Gregorian,
            date_patterns: [
                DatePattern {
                    nr: 1,
                    first_date: None,
                    date: DateBricks {
                        month_word: Option::from(false),
                        day: Option::from(DateInfo { nr: 1, name: DateName::Day }),
                        month: Option::from(DateInfo { nr: 2, name: DateName::Month }),
                        year: Option::from(DateInfo { nr: 3, name: DateName::Year }),
                    },
                },
                DatePattern {
                    nr: 2,
                    first_date: None,
                    date: DateBricks {
                        month_word: Option::from(true),
                        day: Option::from(DateInfo { nr: 1, name: DateName::Day }),
                        month: Option::from(DateInfo { nr: 2, name: DateName::Month }),
                        year: Option::from(DateInfo { nr: 3, name: DateName::Year }),
                    },
                },
            ].to_vec(),
        };
        todo!()
        /*
        let mut data_column  = TransientDataCol::new();
        data_column.column = vec_1;
        let result = _to_date(&data_column, &date_method.date_patterns, &date_method.date_type).unwrap();
        assert_eq!(result.column, ["GREGORIAN:CE:1991:01:01:CE:1991:01:01", "GREGORIAN:CE:0400:02:03:CE:0400:02:03", "GREGORIAN:CE:1991:01:02:CE:1991:01:02"]);

         */
    }
}
