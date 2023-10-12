use std::collections::HashMap;


type AttributeMap = HashMap<String,String>;

struct ElementData{
    tag_name: String,
    attributes: AttributeMap,
}
enum NodeType {
    Text(String),
    Element(ElementData),
}
struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

fn text(text: String) -> Node {
    Node{
        children: Vec::new(),node_type:  NodeType::Text(text),
    }
}

fn element(name: String, attrs: AttributeMap, children: Vec<Node>) -> Node {
    Node { children: children, node_type: NodeType::Element(
        ElementData{
            tag_name: name,
            attributes: attrs,
        }
    ) }
}
