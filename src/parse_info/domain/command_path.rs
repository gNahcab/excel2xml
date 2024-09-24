use crate::parse_info::domain::command::ParseInfoCommand;

pub enum CommandOrPath {
    Path(String),
    Command(ParseInfoCommand)
}

impl CommandOrPath {
    pub(crate) fn new_path(path: String) -> Self {
         Self::Path(path)
    }
    pub(crate) fn new_command(command: ParseInfoCommand) -> Self {
        Self::Command(command)
    }
}