use std::collections::HashMap;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_info::domain::parse_info::ParseInformation;
use crate::parse_info::domain::prop_supplement::PropSupplement;
use crate::parse_info::domain::resource_supplement::ResourceSupplement;
use crate::parse_xlsx::domain::data_col::DataCol;
use crate::parse_xlsx::domain::data_header::{discern_label_id_propnames_and_supplements, DataHeader, DataHeaderWrapper};
use crate::parse_xlsx::domain::data_row::DataRow;
use crate::parse_xlsx::domain::header::Header;
use crate::parse_xlsx::domain::instance::{InstanceWrapper, Instance};
use crate::parse_xlsx::domain::updated_data_sheet::UpdatedDataSheet;
use crate::parse_xlsx::errors::ExcelDataError;

pub struct DataContainer {
    pub res_name: String,
    pub data_header: DataHeader,
    pub resources: Vec<Instance>
}

impl DataContainer {
    fn new(data_header: DataHeader, data_resources: Vec<Instance>, res_name: String) -> Self {
        DataContainer{data_header, res_name,resources:  data_resources}
    }
}

pub struct DataContainerWrapper (pub(crate) UpdatedDataSheet);

impl DataContainerWrapper {
    pub(crate) fn to_data_container(&self, data_model: &DataModel, parse_info: &ParseInformation) -> Result<DataContainer, ExcelDataError> {
        let mut data_instances: Vec<Instance> = vec![];
        let (rows, col_nr_to_row_nr) = _to_rows(&self.0.col_nr_to_cols);
        let supplements = parse_info.res_name_to_supplements.get(self.0.res_name.as_str());
        /*
        let supplements = match parse_info.res_name_to_supplements.get(self.0.res_name.as_str()) {
            None => {
                return Err(ExcelDataError::InputError(format!("Res-name '{}' does not exist in res_name_to_supplements. Other possible values in res_name_to_supplements: '{:?}'", self.0.res_name, parse_info.res_name_to_supplements.iter().map(|(res_name, _)|res_name).collect::<Vec<&String>>())));
            }
            Some(supplements) => {supplements}
        };*/

        let(col_nr_to_propname, col_nr_to_prop_suppl, col_nr_to_res_suppl, col_nr_to_id_label) = discern_label_id_propnames_and_supplements(&self.0.header_to_col_nr, &data_model.properties, supplements)?;

        let (row_nr_to_propname, row_nr_to_prop_suppl, row_nr_to_res_suppl, row_nr_to_id_label) = change_col_nr_to_row_nr(col_nr_to_propname, col_nr_to_prop_suppl, col_nr_to_res_suppl, col_nr_to_row_nr, col_nr_to_id_label);

        let super_field = &data_model.resources.iter().find(|dm_res|dm_res.name.eq(&self.0.res_name)).unwrap().super_field;
        let data_header = DataHeaderWrapper(self.0.header_to_col_nr.to_owned()).to_data_header(&data_model, &self.0.res_name, &row_nr_to_propname, &row_nr_to_prop_suppl, &row_nr_to_res_suppl, &row_nr_to_id_label)?;
        for row in rows.iter() {
            data_instances.push(InstanceWrapper(row.to_owned()).to_instance(&data_model, &parse_info.separator, &row_nr_to_propname, &row_nr_to_prop_suppl, &row_nr_to_res_suppl, &row_nr_to_id_label, super_field, parse_info.set_permissions)?);
        }
        Ok(DataContainer::new(data_header, data_instances, self.0.res_name.to_owned()))
    }
}
fn change_col_nr_to_row_nr(col_nr_to_propname: HashMap<usize, String>, col_nr_to_prop_suppl: HashMap<usize, PropSupplement>, col_nr_to_res_suppl: HashMap<usize, ResourceSupplement>, col_nr_to_row_nr: HashMap<usize, usize>, col_nr_to_id_label: HashMap<usize, Header>) -> (HashMap<usize, String>, HashMap<usize, PropSupplement>, HashMap<usize, ResourceSupplement>, HashMap<usize, Header>) {
    (
        col_nr_to_propname.iter().map(|(col_nr, propname)| (col_nr_to_row_nr.get(col_nr).unwrap().to_owned(), propname.to_owned())).collect(),
        col_nr_to_prop_suppl.iter().map(|(col_nr, header)| (col_nr_to_row_nr.get(col_nr).unwrap().to_owned(), header.to_owned())).collect(),
        col_nr_to_res_suppl.iter().map(|(col_nr, header)| (col_nr_to_row_nr.get(col_nr).unwrap().to_owned(), header.to_owned())).collect(),
        col_nr_to_id_label.iter().map(|(col_nr, header)| (col_nr_to_row_nr.get(col_nr).unwrap().to_owned(), header.to_owned())).collect()
    )
}

fn _to_rows(col_nr_to_cols: &HashMap<usize, DataCol>) -> (Vec<DataRow>, HashMap<usize, usize>) {
    let mut rows = vec![];
    // it is supposed we have at least one col, if no columns exist it breaks here
    let row_length = col_nr_to_cols.get(&0).unwrap().col.len();
    for _ in 0..row_length {
        rows.push(DataRow::new());
    }
    // dictionary makes sure the correct column corresponds to the respective row
    let mut col_nr_to_row_nr: HashMap<usize, usize> = HashMap::new();

    for (row_nr, (col_nr, col)) in col_nr_to_cols.iter().enumerate() {
        col_nr_to_row_nr.insert(col_nr.to_owned(), row_nr);
        for (curr, value) in col.col.iter().enumerate() {
            rows[curr].add_data(value.to_owned());
        }
    }
    (rows, col_nr_to_row_nr)
}

