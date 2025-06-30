use std::ffi::OsStr;
use std::path::PathBuf;
use crate::parse_hcl::domain::command_path::CommandOrPath;
use crate::path_operations::canonicalize_path::{canonicalize_path, find_datamodel};
use crate::path_operations::errors::PathOpError;

pub(crate) fn canonicalize_paths(dm_path: &CommandOrPath, folder_data_path: &PathBuf, hcl_path: PathBuf) -> Result<(PathBuf, PathBuf), PathOpError> {
    // should update the datamodel and data-folder path
    let new_dm_path = match dm_path {
        CommandOrPath::Path(datamodel_path) => {
            canonicalize_path(datamodel_path)?
        }
        CommandOrPath::Command(command) => {
            match command { crate::parse_hcl::domain::command::ParseInfoCommand::FINDPaths => {
                find_datamodel(hcl_path)?
            } }
        }
    };
    let new_folder_data_path = canonicalize_path(folder_data_path)?;
    Ok((new_folder_data_path, new_dm_path))
}
pub fn filter_paths_based_on_extension(dir: &PathBuf, extension_to_check: &str) -> Result<Vec<PathBuf>, PathOpError>{
    // extension should contain no point (e.g. extension is 'json', not '.json')
    let dir = match dir.read_dir() {
        Ok(dir) => {dir}
        Err(err) => {
            return Err(PathOpError::IO(err));
        }
    };

    let mut filtered_paths = vec![];
    for path in dir {
        match path {
            Ok(path) => {
                let extension = match path.path().extension() {
                    None => {
                        // ignore
                        continue;
                    }
                    Some(extension) => {extension.to_owned()}
                };
                if extension.eq(extension_to_check) {
                    filtered_paths.push(path.path());
                }
            }
            Err(err) => {
                PathOpError::IO(err);
            }
        }
    }
    Ok(filtered_paths)
}