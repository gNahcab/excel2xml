use crate::errors::Excel2XmlError;




     /*
    // parse raw dataclusters into trustful data resources
    let mut  resource_name_to_resources: HashMap<String, Vec<DataResource>> = Default::default();
    for datasheet in &datasheets {
        let mut data_resources: Vec<DataResource> = vec![];
        for raw_resource in datasheet.tabular_data.iter() {
            let data_resource  = DataResourceWrapper(raw_resource.to_owned()).to_data_resource(&data_model, separator.to_string())?;
            data_resources.push(data_resource);
        }
        resource_name_to_resources.insert(datasheet.resource_name.to_string(), data_resources);
    }
    */
    // finally we turn the datasheet-resources into xml-code
