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
pub struct Node {}

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
    namespace_decls: Vec<NamespaceDecl>,
    namespace_uri: String,
    name: String,
    attributes: Vec<Attribute>,
    child: Vec<Node>,
}

#[derive(Clone, Debug)]
pub struct XmlDom {
    file: ResourceFile,
    string_pool: StringPool,
    root: Box<Element>,
}

impl XmlDom {}
