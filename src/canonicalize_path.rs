use std::fs;
use std::path::PathBuf;
use crate::parse_info::domain::command_path::CommandOrPath;
use crate::parse_info::errors::HCLDataError;

    pub(crate) fn canonicalize_paths(dm_path: &CommandOrPath, folder_data_path: &PathBuf, hcl_path: PathBuf) -> Result<(PathBuf, PathBuf), HCLDataError> {
        // should update the datamodel and data-folder path
        let new_dm_path = match dm_path {
            CommandOrPath::Path(datamodel_path) => {
                canonicalize_path(datamodel_path)?
            }
            CommandOrPath::Command(command) => {
                match command { crate::parse_info::domain::command::ParseInfoCommand::FINDPaths => {
                    find_datamodel(hcl_path)?
                } }
            }
        };
        let new_folder_data_path = canonicalize_path(folder_data_path)?;
        Ok((new_folder_data_path, new_dm_path))
    }
fn canonicalize_path(path: &PathBuf) -> Result<PathBuf, HCLDataError> {
    match fs::canonicalize(&path) {
        Ok(full_path) => { Ok(full_path) }
        Err(err) => {
             Err(HCLDataError::InputError(format!("Error while looking for absolute path of '{:?}'. Error message was: '{:?}'", path, err)))
        }
    }
}
fn find_datamodel(hcl_path: PathBuf) -> Result<PathBuf, HCLDataError> {
    // look in folder hcl is in, else not found
    let mut dir = hcl_path.to_owned();
    dir.pop();
    match find_json_datamodel(&dir) {
        None => {
            Err(HCLDataError::ParsingError(format!("Looking for datamodel in folder with hcl-file '{:?}'; but couldn't find datamodel there.", dir)))
        }
        Some(dm_path) => {Ok(dm_path)}
    }
}

fn find_json_datamodel(curr_dir: &PathBuf) -> Option<PathBuf> {
    let dir_reader = curr_dir.read_dir().expect("Expected to be able to read directory");
    let paths = dir_reader.
        map(|dir_entry|dir_entry).
        filter(|dir_entry|dir_entry.is_ok())
        .map(|dir_entry| dir_entry.unwrap().path())
        .filter_map(|path|
            if path.extension().map_or(false, |ext| ext == "json") {
                Some(path)
            } else {
                None
            }
        )
        .collect::<Vec<_>>();
    if paths.len() != 1 {
        // Success only if one and only one json-file exists in this folder
        None
    } else {
        Some(paths.get(0).unwrap().to_owned())
    }
}

