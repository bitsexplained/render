
    use std::collections::HashMap;
    type AttributeMap = HashMap<String,String>;
    pub struct ElementData{
        tag_name: String,
        attributes: AttributeMap,
    }
    pub enum NodeType {
        Text(String),
        Element(ElementData),
    }
    pub struct Node {
        children: Vec<Node>,
        node_type: NodeType,
    }

    pub fn text(text: String) -> Node {
        Node{
            children: Vec::new(),node_type:  NodeType::Text(text),
        }
    }

    pub fn element(name: String, attrs: AttributeMap, children: Vec<Node>) -> Node {
        Node { children: children, node_type: NodeType::Element(
            ElementData{
                tag_name: name,
                attributes: attrs,
            }
        ) }
    }


