use std::collections::{HashMap, HashSet};
use crate::parse_hcl::domain::xlsx_sheet_info::{SheetInfo, SheetInfoWrapper};
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::methods_domain::create_method::CreateMethod;
use crate::parse_hcl::wrapper_trait::Wrapper;

pub struct XLSXWorbookInfo {
    pub rel_path:String,
    pub sheet_infos: HashMap<usize, SheetInfo>,
}

impl XLSXWorbookInfo {
        fn new(transient_xlsxworkbook: TransientXLSXWorkbook) -> Self{
            XLSXWorbookInfo { rel_path: transient_xlsxworkbook.rel_path, sheet_infos: transient_xlsxworkbook.sheet_infos}
    }
}

pub(crate) struct XLSXWorkbookInfoWrapper(pub(crate) hcl::Block);

struct TransientXLSXWorkbook {
    rel_path: String,
    sheet_infos: HashMap<usize, SheetInfo>,
}

impl TransientXLSXWorkbook {
    pub(crate) fn no_duplicates(&self) -> Result<(), HCLDataError> {
        self.no_duplicate_res_names()?;
        self.no_duplicate_output_names()?;
        Ok(())
    }
}

impl TransientXLSXWorkbook {
    fn new() -> Self {
        TransientXLSXWorkbook { rel_path: "".to_string(), sheet_infos: Default::default() }
    }
    pub(crate) fn add_rel_path(&mut self, rel_path: String) {
        self.rel_path = rel_path;
    }
    pub(crate) fn add_sheet_info(&mut self, sheet_info: SheetInfo) -> Result<(), HCLDataError> {
        if self.sheet_infos.contains_key(&sheet_info.sheet_nr) {
            return Err(HCLDataError::InputError(format!("Found")));
        }
        self.sheet_infos.insert(sheet_info.sheet_nr, sheet_info);
        Ok(())

    }

    fn check_for_duplicates(data: Vec<&String>) -> Option<String> {
        let mut uniq: HashSet<String> = HashSet::new();
        for value in data.iter() {
            if !uniq.insert(value.to_string()) {
                return Some(value.to_string())
            }
        }
        None
    }
    fn no_duplicate_output_names(&self) -> Result<(), HCLDataError> {
        // check that all output-names in assignments, supplements and transform don't have any duplicates
        let mut output_names = vec![];
        for (_, sheet_info) in self.sheet_infos.iter() {
            if sheet_info.supplements.is_some() {
                sheet_info.supplements.as_ref().unwrap().header_to_prop_suppl.iter().for_each(|(header,_)|output_names.push(header));
                sheet_info.supplements.as_ref().unwrap().header_to_res_suppl.iter().for_each(|(header,_)|output_names.push(header));
            }
            sheet_info.assignments.propname_to_header.iter().for_each(|(propname, _)|output_names.push(propname));
            if sheet_info.transformations.is_some() {
                let transformations = sheet_info.transformations.as_ref().unwrap();
                transformations.replace_label_name_methods.iter().for_each(|method|output_names.push(&method.output));
                transformations.replace_methods.iter().for_each(|method|output_names.push(&method.output));
                transformations.alter_methods.iter().for_each(|method|output_names.push(&method.output));
                for create_method in transformations.create_methods.iter() {
                    match create_method {
                        CreateMethod::IntegerCreateMethod(int_create) => {
                            output_names.push(&int_create.output);
                        }
                        CreateMethod::PermissionsCreateMethod(permissions_create) => {
                            output_names.push(&permissions_create.output);
                        }
                    }

                }
                transformations.identify_methods.iter().for_each(|method|output_names.push(&method.output));
                transformations.to_date_methods.iter().for_each(|method|output_names.push(&method.output));
                transformations.combine_methods.iter().for_each(|method|output_names.push(&method.output));
                transformations.upper_methods.iter().for_each(|method|output_names.push(&method.output));
                transformations.lower_methods.iter().for_each(|method|output_names.push(&method.output));
                transformations.replace_with_iri.iter().for_each(|method|output_names.push(&method.output));

            }
        }
        match Self::check_for_duplicates(output_names){
            None => {
                //ignore
            }
            Some(duplicate) => {
                return Err(HCLDataError::InputError(format!("Duplicate prop-names '{}' found in sheet-infos of xlsx '{}'", duplicate, self.rel_path)));
            }
        }
        Ok(())


    }
    fn no_duplicate_res_names(&self) -> Result<(), HCLDataError> {
        let resource_names = self.sheet_infos.iter().map(|(_, sheet_info)|&sheet_info.resource_name).collect::<Vec<_>>();
        match Self::check_for_duplicates(resource_names){
            None => {
                //ignore
                }
            Some(duplicate) => {
                return Err(HCLDataError::InputError(format!("Duplicate resource-names '{}' found in sheet-infos of xlsx '{}'", duplicate, self.rel_path)));
                }
        }
        Ok(())
    }
}


impl XLSXWorkbookInfoWrapper {
    pub(crate) fn to_wb_info(&self) -> Result<XLSXWorbookInfo, HCLDataError> {
        let rel_path = self.0.get_single_label()?;
        let mut transient_xlsx_workbook = TransientXLSXWorkbook::new();
        transient_xlsx_workbook.add_rel_path(rel_path);
        let blocks: Vec<&hcl::Block> = self.0.blocks();
        self.0.no_attributes()?;
        for block in blocks {
            match block.identifier.as_str() {
                "sheet" => {
                    let sheet: SheetInfo = SheetInfoWrapper(block.to_owned()).to_sheet_info()?;
                    transient_xlsx_workbook.add_sheet_info(sheet)?;
                }
                _ => {
                    return Err(HCLDataError::InputError(format!("parse-info-hcl: only 'sheet' is allowed as block-identifier on first level of 'xlsx'-block. Wrong identifier: {}",block.identifier.as_str())));
                } }
        }
        transient_xlsx_workbook.no_duplicates()?;
       Ok(XLSXWorbookInfo::new(transient_xlsx_workbook))
    }
}

