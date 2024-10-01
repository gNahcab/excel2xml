use std::collections::HashMap;
use hcl::{Attribute, Expression, Identifier};
use crate::parse_info::domain::command::{ParseInfoCommand, ParseInfoCommandWrapper};
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::wrapper_trait::Wrapper;
use crate::parse_xlsx::domain::data_sheet::DataSheetWrapper;

pub struct Assignments {
    pub header_to_propname: HashMap<String, String>,
    find_rest: bool
}

impl Assignments {
    fn new(transient_assignments: TransientAssignments) -> Self{
        Assignments{ header_to_propname: transient_assignments.header_to_propname, find_rest: transient_assignments.find_rest.unwrap()}
    }
}
struct TransientAssignments {
    header_to_propname: HashMap<String, String>,
    find_rest: Option<bool>
}

impl TransientAssignments {
}

impl TransientAssignments {
    fn new() -> Self {
        TransientAssignments{ header_to_propname: Default::default(), find_rest: None }
    }
    fn add_command(&mut self, command: ParseInfoCommand) -> Result<(), HCLDataError> {
        match command { ParseInfoCommand::FINDPaths => {
            if self.find_rest.is_some() {
                return Err(HCLDataError::InputError(format!("duplicate command cmd-find in assignments. Assignments already collected: '{:?}'", self.header_to_propname)));
            }
            self.find_rest = Option::Some(true);
        } }
        Ok(())
    }
     fn add_header_to_prop_name(&mut self, header: String, prop_name: String) -> Result<(), HCLDataError> {
        if self.header_to_propname.contains_key(header.as_str()) {
            return Err(HCLDataError::ParsingError(format!("duplicate header '{}' found in assignments.", header)));
        }
        self.header_to_propname.insert(header.to_string(), prop_name.to_string());
        Ok(())
    }
    fn complete(&mut self) {
        // if attribute find_rest is none set to false
        if self.find_rest.is_none() {
            self.find_rest = Option::Some(false);
        }
    }
}
pub(crate) struct AssignmentsWrapper(pub(crate) hcl::Block);

impl AssignmentsWrapper {
    pub(crate) fn to_assignments(&self) -> Result<Assignments, HCLDataError> {
        let mut transient_assignments = TransientAssignments::new();
        for attribute in self.0.attributes() {
            // filter special key-id 'rest', else normal header of xlsx
            match attribute.key.as_str() {
                "rest" => {
                    match &attribute.expr {
                        Expression::String(prop_name) => {
                            // this means a header of the xlsx is called "rest"
                            transient_assignments.add_header_to_prop_name(attribute.key.to_string(), prop_name.to_owned())?;
                        }
                        // this means a command is given
                        Expression::Traversal(traversal) => {
                            let command: ParseInfoCommand = ParseInfoCommandWrapper(traversal.to_owned()).to_command()?;
                            transient_assignments.add_command(command)?;
                        }
                        _ => {
                            return Err(HCLDataError::InputError(format!("only values of type String or Traversal are allowed in assignments with id 'rest', but found: '{:?}' with key: '{:?}'", attribute, attribute.key)));
                        }
                    }
                }
                _ => {
                    // normal header
                    match &attribute.expr {
                        Expression::String(prop_name) => {
                            transient_assignments.add_header_to_prop_name(attribute.key.to_string(), prop_name.to_owned())?;
                        }
                        _ => {
                            return Err(HCLDataError::InputError(format!("only values of type String are allowed in assignments (exception with 'rest'), but found: '{:?}' with key: '{:?}'", attribute, attribute.key)));
                        }
                    }
                }
            }

        }
        transient_assignments.complete();
        Ok(Assignments::new(transient_assignments))
    }
}

