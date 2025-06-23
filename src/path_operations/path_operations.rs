use std::path::PathBuf;
use crate::parse_info::domain::command_path::CommandOrPath;
use crate::path_operations::canonicalize_path::{canonicalize_path, find_datamodel};
use crate::path_operations::errors::PathOpError;

pub(crate) fn canonicalize_paths(dm_path: &CommandOrPath, folder_data_path: &PathBuf, hcl_path: PathBuf) -> Result<(PathBuf, PathBuf), PathOpError> {
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
pub fn filter_paths_of_dir(dir: &PathBuf, ending: &str) -> Result<Vec<PathBuf>, PathOpError>{
    let mut xlsx_paths = vec![];
    let dir = match dir.read_dir() {
        Ok(dir) => {dir}
        Err(err) => {
            return Err(PathOpError::IO(err))
        }
    };
    for path in dir {
        match path {
            Ok(path) => {
                if path.path().ends_with(ending) {
                    xlsx_paths.push(path.path());
                }
            }
            Err(err) => {
                PathOpError::IO(err);
            }
        }
    }
    Ok(xlsx_paths)
}
