use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_xlsx::domain::data_header::{DataHeader, DataHeaderWrapper};
use crate::parse_xlsx::domain::data_resource::{DataResource, DataResourceWrapper};
use crate::parse_xlsx::domain::data_sheet::DataSheet;
use crate::parse_xlsx::errors::ExcelDataError;

pub struct DataContainer {
    pub res_name: String,
    pub data_header: DataHeader,
    pub resources: Vec<DataResource>
}

impl DataContainer {
    fn new(data_header: DataHeader, data_resources: Vec<DataResource>, res_name: String) -> Self {
        DataContainer{res_name, data_header,resources:  data_resources}
    }
}

pub struct DataContainerWrapper (pub(crate) DataSheet);

impl DataContainerWrapper {
    pub(crate) fn to_data_container(&self, data_model: &DataModel, separator: &String) -> Result<DataContainer, ExcelDataError> {
        let data_header: DataHeader = DataHeaderWrapper(self.0.headers.to_owned()).to_data_header(&data_model, &self.0.res_name)?;
        let mut data_resources: Vec<DataResource> = vec![];
        for (nr, row) in self.0.data_rows.iter().enumerate() {
            data_resources.push(DataResourceWrapper(row.to_owned()).to_data_resource(data_model, separator, &data_header, nr)?);
        }
        Ok(DataContainer::new(data_header, data_resources, self.0.res_name.to_owned()))
    }
}

