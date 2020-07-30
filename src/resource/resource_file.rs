use crate::resource::config_description::ConfigDescription;
use crate::resource::source::Source;
use crate::resource::ResourceType;

#[derive(Clone, Debug)]
pub struct ResourceName {
    package: String,
    typ: ResourceType,
    entry: String,
}

impl ResourceName {
    pub fn new(resource_type: ResourceType) -> ResourceName {
        ResourceName {
            package: "".to_string(),
            typ: resource_type,
            entry: "".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}/{}", self.package, self.typ, self.entry)
    }
}

#[derive(Clone, Debug)]
pub enum ResourceFileType {
    kUnknown,
    kPng,
    kBinaryXml,
    kProtoXml,
}

#[derive(Clone, Debug)]
pub struct SourcedResourceName {
    name: ResourceName,
    line: usize,
}

#[derive(Clone, Debug)]
pub struct ResourceFile {
    pub(crate) name: ResourceName,
    pub(crate) typ: ResourceFileType,
    pub(crate) source: Source,
    pub(crate) config: ConfigDescription,
    pub(crate) exported_symbols: Vec<SourcedResourceName>,
}

impl ResourceFile {
    pub fn new() -> ResourceFile {
        ResourceFile {
            name: ResourceName::new(ResourceType::None),
            typ: ResourceFileType::kUnknown,
            source: Source::new(),
            config: ConfigDescription::new(),
            exported_symbols: vec![],
        }
    }
}
