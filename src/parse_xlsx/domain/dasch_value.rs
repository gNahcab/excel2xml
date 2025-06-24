use std::cmp::PartialEq;
use clap::builder::Str;
use crate::parse_dm::domain::gui_element::GUIElement;
use crate::parse_dm::domain::object::ValueObject;
use crate::parse_xlsx::domain::dasch_value_field::TransientSupplementValueField;
use crate::parse_xlsx::domain::encoding::Encoding;
use crate::parse_xlsx::domain::permissions::Permissions;
use crate::parse_xlsx::errors::ExcelDataError;

pub struct TransientDaschValue {
    pub value: String,
    pub permission: Option<Permissions>,
    pub encoding: Option<Encoding>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DaschValue {
    pub value: String,
    pub permission: Option<Permissions>,
    pub encoding: Option<Encoding>,
    pub comment: Option<String>,
}
impl DaschValue {
    pub(crate) fn new(transient_dasch_value: TransientDaschValue) -> Self {
        Self{
            value: transient_dasch_value.value,
            permission: transient_dasch_value.permission,
            encoding: transient_dasch_value.encoding,
            comment: transient_dasch_value.comment,
        }
    }
}

impl PartialEq for Encoding {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl TransientDaschValue {
    pub(crate) fn new(value: String) -> Self {
        TransientDaschValue {
            value,
            permission: None,
            encoding: None,
            comment: None,
        }
    }
    pub(crate) fn add_encoding(&mut self, encoding: Encoding, gui_element: &GUIElement) -> Result<(), ExcelDataError> {
        match gui_element {
            GUIElement::RICHTEXT => {
                if encoding != Encoding::XML {
                    return Err(ExcelDataError::InputError(format!("DaSCH-Value-Error: Encoding of '{}' should be xml, since Gui-Element is 'Richtext', but found: {}", self.value, encoding)));
                }
            }
            GUIElement::SIMPLETEXT => {
                if encoding != Encoding::UTF8 {
                    return Err(ExcelDataError::InputError(format!("DaSCH-Value-Error: Encoding of '{}' should be utf8, since Gui-Element is 'Simpletext', but found: {}", self.value, encoding)));
                }
            }
            GUIElement::LIST => {}
            GUIElement::DATE => {}
            GUIElement::SEARCHBOX => {}
            GUIElement::GEONAMES => {}
            GUIElement::TEXTAREA => {
                if encoding != Encoding::UTF8 {
                    return Err(ExcelDataError::InputError(format!("DaSCH-Value-Error: Encoding of '{}' should be utf8, since Gui-Element is 'TextArea', but found: {}", self.value,  encoding)));
                }
            }
        }
        self.encoding = Some(encoding);
        Ok(())
    }
    pub(crate) fn add_comment(&mut self, comment: String) {
        self.comment = Some(comment);
    }
    pub(crate) fn add_permissions(&mut self, permission: Permissions) {
        self.permission = Some(permission);
    }
    pub(crate) fn complete(&mut self, gui_element: &GUIElement, set_permissions: bool) -> Result<(), ExcelDataError> {
        if self.permission.is_none() {
            if set_permissions {
                // set default
                self.permission = Some(Permissions::DEFAULT);
            }
            //return Err(ExcelDataError::InputError(format!("Permissions of DaschValue '{}' is None.", self.value)))

        }
        match gui_element {
            GUIElement::RICHTEXT => {
                    if self.encoding.is_none() {
                        //return Err(ExcelDataError::InputError(format!("Encoding of '{}' is None, but it is a TextValue.", self.value)));
                        // set default (i.e. utf8)
                        self.encoding = Some(Encoding::XML);
                    }
            }
            GUIElement::SIMPLETEXT => {
                if self.encoding.is_none() {
                    //return Err(ExcelDataError::InputError(format!("Encoding of '{}' is None, but it is a TextValue.", self.value)));
                    // set default (i.e. utf8)
                    self.encoding = Some(Encoding::UTF8);
                }
            }
            GUIElement::LIST => {}
            GUIElement::DATE => {}
            GUIElement::SEARCHBOX => {}
            GUIElement::GEONAMES => {}
            GUIElement::TEXTAREA => {
                if self.encoding.is_none() {
                    //return Err(ExcelDataError::InputError(format!("Encoding of '{}' is None, but it is a TextValue.", self.value)));
                    // set default (i.e. utf8)
                    self.encoding = Some(Encoding::UTF8);
                }
            }
        }
        Ok(())
    }
}

pub struct WrapperDaschValue(pub String);
impl WrapperDaschValue {
    pub(crate) fn to_dasch_value(&self, pos: usize, maybe_suppl_value: Option<&TransientSupplementValueField>, gui_element: &GUIElement, set_permissions: bool) -> Result<DaschValue, ExcelDataError> {
        let mut transient_dasch_value = TransientDaschValue::new(self.0.to_owned());
        if maybe_suppl_value.is_some() {
            if maybe_suppl_value.as_ref().unwrap().encoding.is_some() {
                let encoding = match maybe_suppl_value.as_ref().unwrap().encoding.as_ref().unwrap().get(pos) {
                    None => {
                        return Err(ExcelDataError::InputError(format!("Differing length of values and encoding. Values: '{:?}', encoding:'{:?}'.", &self.0, maybe_suppl_value.as_ref().unwrap().encoding.as_ref())))
                    }
                    Some(encoding) => {encoding}
                };
                transient_dasch_value.add_encoding(encoding.to_owned(), gui_element)?;
            }
            if maybe_suppl_value.as_ref().unwrap().comment.is_some() {
                let comment = match maybe_suppl_value.as_ref().unwrap().comment.as_ref().unwrap().get(pos) {
                    None => {
                        return Err(ExcelDataError::InputError(format!("Differing length of values and comment. Values: '{:?}', comment:'{:?}'.", &self.0, maybe_suppl_value.as_ref().unwrap().comment.as_ref())))
                    }
                    Some(comment) => {comment}
                };
                transient_dasch_value.add_comment(comment.to_owned());
            }
            if maybe_suppl_value.as_ref().unwrap().permissions.is_some() {
                let permissions = match maybe_suppl_value.as_ref().unwrap().permissions.as_ref().unwrap().get(pos) {
                    None => {
                        return Err(ExcelDataError::InputError(format!("Differing length of values and permissions. Values: '{:?}', permissions:'{:?}'.", &self.0, maybe_suppl_value.as_ref().unwrap().permissions.as_ref())))
                    }
                    Some(permissions) => {permissions}
                };
                transient_dasch_value.add_permissions(permissions.to_owned());
            }
        }
        transient_dasch_value.complete(gui_element, set_permissions)?;
        Ok(DaschValue::new(transient_dasch_value))
    }
}