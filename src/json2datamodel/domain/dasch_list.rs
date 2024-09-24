use crate::transfrom2datamodel::domain::label::{Label, LabelWrapper};
use serde_json::Value;
use crate::transfrom2datamodel::errors::DataModelError;

#[derive(Debug, PartialEq, Clone)]
pub struct DaSCHList {
    pub name: String,
    labels: Vec<Label>,
    nodes: Vec<ListNode>,
}

impl DaSCHList {
    fn new(name: Option<String>, labels: Vec<Label>, nodes: Vec<ListNode>) -> DaSCHList {
        DaSCHList{ name: name.unwrap(),labels, nodes }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ListNode {
    name: String,
    labels: Vec<Label>,
    nodes: Vec<ListNode>
}

impl ListNode {
    fn new(name: Option<String>, labels: Vec<Label>, nodes: Vec<ListNode>) -> ListNode {
        ListNode{
            name: name.unwrap(),
            labels,
            nodes
        }
    }
}

struct TransientListNode {
    name: Option<String>,
    labels: Vec<Label>,
    nodes: Vec<ListNode>
}

impl TransientListNode {
    pub(crate) fn is_complete(&self) -> Result<(), DataModelError> {
        // Node is complete if name and at least one label exists
        if self.name.is_none() {
            return Err(DataModelError::ParsingError(String::from(format!("Missing name field in labels {:?}", self.labels))))
        }
        if self.labels.is_empty() {
            return Err(DataModelError::ParsingError(String::from(format!("Missing labels field in {:?}", self.name))))
        }

        Ok(())
    }
}

impl TransientListNode {

    fn new() -> Self {
        TransientListNode {
            name: None,
            labels: vec![],
            nodes: vec![],
        }
    }

    fn add_name(&mut self, name: String) {
        self.name = Some(name);
    }
    fn add_label(&mut self, label: Label) {
        self.labels.push(label);
    }
    fn add_node(&mut self, node: ListNode) {
        self.nodes.push(node)
    }
}
pub(crate) struct ListNodeWrapper(pub(crate) Value);
impl ListNodeWrapper {
pub fn to_node(&self) -> Result<ListNode, DataModelError> {
    let object = self.0.as_object().expect("node of list should be an object");
    let mut transient = TransientListNode::new();
    for (key, value) in object.iter() {
        match key.as_str() {
            "name" => {
                    transient.add_name(String::from(value.as_str().expect("name to be a string")))
            }
            "labels" => {
                let label_object = value.as_object().expect("labels should be an object");
                for (key, value) in label_object.iter() {
                    let label = LabelWrapper((key.to_owned(), value.to_owned())).to_label()?;
                    transient.add_label(label);
                }
            }
            "nodes" => {
                let nodes_array = value.as_array().expect("node of list should be an array");
                for raw_node in nodes_array.iter() {
                   let list_node = ListNodeWrapper(raw_node.to_owned()).to_node()?;
                    transient.add_node(list_node);
                }
            }
            _ => {
                //do nothing
            }
        }
    }
    transient.is_complete()?;
    Ok(ListNode::new(transient.name, transient.labels, transient.nodes))
}

}
struct TransientDaSCHList {
    name: Option<String>,
    labels: Vec<Label>,
    comments: Vec<Label>,
    nodes: Vec<ListNode>
}

impl TransientDaSCHList {
    fn new() -> Self {
        TransientDaSCHList{
            name: None,
            labels: vec![],
            comments: vec![],
            nodes: vec![],
        }
    }

    fn add_name(&mut self, name: String) {
        self.name = Some(name);
    }
    fn add_label(&mut self, label: Label) {
        self.labels.push(label);
    }

    fn add_comment(&mut self, comment: Label) {
        self.comments.push(comment);
    }
    fn add_node(&mut self, node: ListNode) {
        self.nodes.push(node)
    }
    pub(crate) fn is_complete(&self) -> Result<(), DataModelError> {
        // List is complete if there is a name, at least one label and at least one node
        if self.name.is_none() {
            return Err(DataModelError::ParsingError(String::from(format!("Missing name field in list of labels {:?}", self.labels))));
        }
        if self.labels.is_empty() {
            return Err(DataModelError::ParsingError(String::from(format!("Missing labels field in {:?}", self.name))));
        }
        if self.nodes.is_empty() {
            return Err(DataModelError::ParsingError(String::from(format!("Missing nodes field in {:?}", self.name))));
        }
        Ok(())
    }

}
pub(crate) struct DaSCHListWrapper(pub(crate) Value);
impl  DaSCHListWrapper{
    pub fn to_list(&self) -> Result<DaSCHList, DataModelError> {
        let object = self.0.as_object().expect("list should be an object");
        let mut transient = TransientDaSCHList::new();
        for (key, value) in object.iter() {
            match key.as_str() {
                "name" => {
                    transient.add_name(String::from(value.as_str().expect("name to be a string")))
                }
                "labels" => {
                    let label_object = value.as_object().expect("labels should be an object");
                    for (key, value) in label_object.iter() {
                        let label = LabelWrapper((key.to_owned(), value.to_owned())).to_label()?;
                        transient.add_label(label);
                    }
                }
                "comments" => {
                    let comment_object = value.as_object().expect("comments should be an object");
                    for (key, value) in comment_object.iter() {
                        let comment = LabelWrapper((key.to_owned(), value.to_owned())).to_label()?;
                        transient.add_comment(comment);
                    }

                }
                "nodes" => {
                    let nodes_array = value.as_array().expect("node of list should be an array");
                    for raw_node in nodes_array.iter() {
                        let list_node = ListNodeWrapper(raw_node.to_owned()).to_node()?;
                        transient.add_node(list_node);
                    }

                }
                _ => {}
            }
        }
        transient.is_complete()?;
        Ok(DaSCHList::new(transient.name, transient.comments, transient.nodes))

    }
}
