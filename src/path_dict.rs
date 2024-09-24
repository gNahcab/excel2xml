use std::collections::HashMap;
use std::path::{Path, PathBuf};

struct PathDict {
    data_model: PathBuf,
    resname_2_xlsx_path: HashMap<String,PathBuf>,
    xlsx_path_2_steer_path: HashMap<PathBuf,PathBuf>
}
fn find_paths(path: &Path) -> HashMap<String, PathBuf> {
    // returns datamodel and xlsx-files
    todo!()
}

fn sort_paths(path: &Path) -> HashMap<String, PathBuf> {
    // returns datamodel and xlsx-files sorted by resource and steer files sorted by xlsx files
    todo!()
}