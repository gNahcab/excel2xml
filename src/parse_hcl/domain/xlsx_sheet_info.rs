use std::collections::HashMap;
use hcl::{Block, BlockLabel, Expression};
use crate::parse_hcl::domain::assignments::{Assignments, AssignmentsWrapper};
use crate::parse_hcl::domain::supplements::{Supplements, SupplementsWrapper};
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::transformations::{Transformations, TransformationsWrapper};
use crate::parse_hcl::wrapper_trait::Wrapper;

pub struct SheetInfo {
    pub sheet_nr: usize,
    pub resource_name: String,
    pub assignments: Assignments,
    pub transformations: Option<Transformations>,
    pub supplements: Option<Supplements>
}
impl SheetInfo {
    fn new(transient_sheet_info: TransientSheetInfo) -> Self {
        SheetInfo{
            sheet_nr: transient_sheet_info.sheet_number,
            resource_name: transient_sheet_info.resource_name.unwrap(),
            assignments: transient_sheet_info.assignments.unwrap(),
            transformations: transient_sheet_info.transformations,
            supplements: transient_sheet_info.supplements
        }
    }
}

struct TransientSheetInfo {
    sheet_number: usize,
    resource_name: Option<String>,
    assignments: Option<Assignments>,
    transformations: Option<Transformations>,
    supplements: Option<Supplements>
}

impl TransientSheetInfo {
    fn new(sheet_nr: usize) -> Self {
        TransientSheetInfo{
            sheet_number: sheet_nr,
            resource_name: None,
            assignments: None,
            transformations: None,
            supplements: None,
        }
    }
    pub(crate) fn add_res_name(&mut self, res_name: String) -> Result<(), HCLDataError> {
        if self.resource_name.is_some() {
            return Err(HCLDataError::InputError(format!("multiple resource-names: First: '{}', Second: '{}'", self.resource_name.as_ref().unwrap(), res_name)));
        }
        self.resource_name = Option::Some(res_name);
        Ok(())
    }
    pub(crate) fn add_supplement(&mut self, supplements: Supplements) -> Result<(), HCLDataError> {
        if self.supplements.is_some() {
            return Err(HCLDataError::InputError(format!("multiple supplements: First: '{:?}', Second: '{:?}'", self.supplements, supplements)));
        }
        self.supplements = Some(supplements);
        Ok(())
    }
    pub(crate) fn add_assignments(&mut self, assignments: Assignments) -> Result<(), HCLDataError> {
        if self.assignments.is_some() {
            return Err(HCLDataError::InputError(format!("multiple declaration of assignments in resource with name '{}' in sheet '{}'", self.resource_name.as_ref().unwrap(), self.sheet_number)));
        }
        self.assignments = Option::from(assignments);
        Ok(())
    }
    pub(crate) fn add_transformations(&mut self, transformations: Transformations) -> Result<(), HCLDataError> {
        if self.transformations.is_some() {
            return Err(HCLDataError::InputError(format!("multiple transformations provided: First: '{:?}', Second: '{:?}'", self.transformations.as_ref().unwrap(), transformations)));
        }
        self.transformations = Option::Some(transformations);
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
                    let resource_name = match &attribute.expr {
                        Expression::String(value) => {value}
                        _ => {
                            return Err(HCLDataError::InputError(format!("Value of 'resource' should be a String, but found something else: {:?}", attribute.expr)));
                        }
                    };
                    transient_sheet_info.add_res_name(resource_name.to_owned())?;
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
                "supplements" => {
                    block.no_attributes()?;
                    let supplements = SupplementsWrapper(block.to_owned()).to_supplements()?;
                    transient_sheet_info.add_supplement(supplements)?;
                }
                "transform" => {
                    let transformations = TransformationsWrapper(block.to_owned()).to_transformations()?;
                    transient_sheet_info.add_transformations(transformations)?;
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


