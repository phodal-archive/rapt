use crate::proto::Resources::StringPool;
use crate::resource::resource_file::ResourceFile;

#[derive(Clone, Debug)]
pub struct NamespaceDecl {
    prefix: String,
    uri: String,
    line_number: i32,
    column_number: i32,
}

impl NamespaceDecl {
    pub fn new() -> NamespaceDecl {
        NamespaceDecl {
            prefix: "".to_string(),
            uri: "".to_string(),
            line_number: 0,
            column_number: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AaptAttribute {}

#[derive(Clone, Debug)]
pub struct Item {}

#[derive(Clone, Debug)]
pub struct Text {
    node: Node,
    text: String,
}

#[derive(Clone, Debug)]
pub struct Node {
    parent: Box<Element>,
    line_number: i32,
    column_number: i32,
    comment: String,
}

impl Node {
    pub fn new() {}
}

#[derive(Clone, Debug)]
pub struct Attribute {
    namespace_uri: String,
    name: String,
    value: String,
    compiled_attribute: Option<AaptAttribute>,
    compiled_value: Box<Item>,
}

#[derive(Clone, Debug)]
pub struct Element {
    node: Node,
    namespace_decls: Vec<NamespaceDecl>,
    namespace_uri: String,
    name: String,
    attributes: Vec<Attribute>,
    child: Vec<Node>,
}

#[derive(Clone, Debug)]
pub struct XmlResource {
    file: ResourceFile,
    string_pool: StringPool,
    root: Box<Element>,
}

impl XmlResource {
    pub fn new(file: ResourceFile, element: Element) -> XmlResource {
        XmlResource {
            file,
            string_pool: Default::default(),
            root: Box::new(element),
        }
    }
}
