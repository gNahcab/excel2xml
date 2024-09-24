use std::num::ParseIntError;
use hcl::BlockLabel;
use crate::parse_info::domain::assignments::{Assignments, AssignmentsWrapper};
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::wrapper_trait::Wrapper;

pub struct SheetInfo {
    pub sheet_nr: usize,
    pub resource_name: String,
    pub assignments: Assignments
}
impl SheetInfo {
    fn new(transient_sheet_info: TransientSheetInfo) -> Self {
        SheetInfo{
            sheet_nr: transient_sheet_info.sheet_number,
            resource_name: transient_sheet_info.resource_name.unwrap(),
            assignments: transient_sheet_info.assignments.unwrap(),
        }
    }
}

struct TransientSheetInfo {
    sheet_number: usize,
    resource_name: Option<String>,
    assignments: Option<Assignments>
}

impl TransientSheetInfo {
    fn new(sheet_nr: usize) -> Self {
        TransientSheetInfo{
            sheet_number: sheet_nr,
            resource_name: None,
            assignments: None,
        }
    }
    pub(crate) fn add_res_name(&mut self, res_name: String) -> Result<(), HCLDataError> {
        if self.resource_name.is_some() {
            return Err(HCLDataError::InputError(format!("multiple resource-names: First: '{}', Second: '{}'", self.resource_name.as_ref().unwrap(), res_name)));
        }
        self.resource_name = Option::Some(res_name);
        Ok(())
    }
    pub(crate) fn add_assignments(&mut self, assignments: Assignments) -> Result<(), HCLDataError> {
        if self.assignments.is_some() {
            return Err(HCLDataError::InputError(format!("multiple declaration of assignments in resource with name '{}' in sheet '{}'", self.resource_name.as_ref().unwrap(), self.sheet_number)));
        }
        self.assignments = Option::from(assignments);
        Ok(())
    }
    pub(crate) fn is_complete(&self) -> Result<(), HCLDataError> {
        if self.resource_name.is_none() {
            return Err(HCLDataError::InputError("Resource name is missing.".to_string()))
        }
        if self.assignments.is_none() {
            return Err(HCLDataError::InputError("Assignments is missing.".to_string()))
        }
        Ok(())
    }
}

pub(crate) struct SheetInfoWrapper(pub(crate) hcl::Block);

impl SheetInfoWrapper {
    pub(crate) fn to_sheet_info(&self) -> Result<SheetInfo, HCLDataError> {
        let sheet_nr = self.collect_sheet_nr()?;
        let mut transient_sheet_info = TransientSheetInfo::new(sheet_nr);
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "resource" => {
                    let resource_name = attribute.expr.to_string();
                    transient_sheet_info.add_res_name(resource_name)?;
                }
                _ => {
                    return Err(HCLDataError::InputError(format!("found unknown attribute-key '{}' in sheet-info.", attribute.key.as_str())));
                }
            }
        }
        for block in self.0.blocks() {
            match block.identifier.as_str() {
                "assignments" => {
                    let assignments = AssignmentsWrapper(block.to_owned()).to_assignments()?;
                    transient_sheet_info.add_assignments(assignments)?;
                }
                _ => {
                    return Err(HCLDataError::InputError(format!("found unknown block-identifier '{}' for attribute in sheet-info.", block.identifier.as_str())));
                }
            }

        }
        transient_sheet_info.is_complete()?;
        Ok(SheetInfo::new(transient_sheet_info))
    }
    fn collect_sheet_nr(&self) -> Result<usize, HCLDataError> {
        if self.0.labels.len() != 1 {
            return Err(HCLDataError::InputError(format!("Sheet should have exactly one label, but found: {:?}", self.0.labels)));
        }
        Ok(match self.0.labels.get(0).unwrap() {
            BlockLabel::String(value) => {
                match value.parse::<usize>() {
                    Ok(number) => {number}
                    Err(_) => {
                        return Err(HCLDataError::InputError(format!("cannot parse sheet-nr '{}' to usize", value)));
                    }
                }
            }
            _ => {
                return Err(HCLDataError::InputError(format!("Sheet should have a String as label, but found: {:?}", self.0.labels)));
            }
        })
    }
}
