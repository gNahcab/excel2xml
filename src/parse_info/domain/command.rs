use std::fmt::{Debug, Pointer};
use hcl::TraversalOperator;
use crate::hcl_info::errors::HCLDataError;

#[derive(Debug)]
pub(crate) enum ParseInfoCommand {
    // find all or some paths in the directory
    FINDPaths,
}

impl ParseInfoCommand {
    fn parse_info_command(name: String) -> Result<ParseInfoCommand, HCLDataError> {
        match name.as_str() {
            "find" => { Ok(ParseInfoCommand::FINDPaths) }
            _ => {
                Err(HCLDataError::InputError(format!("cannot find ParseInfoCommand: '{}'", name)))
            }
        }
    }
}

pub(crate) struct ParseInfoCommandWrapper (pub(crate) Box<hcl::Traversal>);
impl ParseInfoCommandWrapper {
    pub(crate) fn to_command(&self) -> Result<ParseInfoCommand, HCLDataError> {
        let cmd = self.0.expr.to_string();
        if cmd != "cmd" {
            return Err(HCLDataError::ParsingError(format!("command can only be 'cmd'. But found: {}", cmd)));
        }
        let operators:Vec<&hcl::TraversalOperator> = self.0.operators.iter().collect();
        if operators.len() != 1 {
            return Err(HCLDataError::ParsingError(format!("cmd-command should only have one operato, rnot zero or multiple operators. Found: '{:?}'", operators)));
        }
        let command = operators.get(0).unwrap();
        let name = match command {
            TraversalOperator::GetAttr(name) => {
                name.to_string()
            }
            _ => {
                return Err(HCLDataError::InputError(format!("not implemented this type of TraversalOperator, invalid command: '{:?}'", command)));
            }
        };
        ParseInfoCommand::parse_info_command(name)
    }
}
