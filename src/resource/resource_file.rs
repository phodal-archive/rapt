use crate::resource::config_description::ConfigDescription;
use crate::resource::source::Source;
use crate::resource::ResourceType;

#[derive(Clone, Debug)]
pub struct ResourceName {
    package: String,
    typ: ResourceType,
    entry: String,
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
    name: ResourceName,
    typ: ResourceFileType,
    source: Source,
    config: ConfigDescription,
    exported_symbols: Vec<SourcedResourceName>,
}
