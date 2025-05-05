use std::path::{Path, PathBuf};
use crate::hcl_info::domain::command::ParseInfoCommand;

pub enum CommandOrPath {
    Path(PathBuf),
    Command(ParseInfoCommand)
}

impl CommandOrPath {
    pub(crate) fn new_path(path: PathBuf) -> Self {
         Self::Path(path)
    }
    pub(crate) fn new_command(command: ParseInfoCommand) -> Self {
        Self::Command(command)
    }
}