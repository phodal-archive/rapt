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
    pub(crate) name: ResourceName,
    pub(crate) line: usize,
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

#[cfg(test)]
mod tests {
    use crate::resource::resource_file::ResourceName;
    use crate::resource::ResourceType;

    #[test]
    fn should_success_convert_resource_name() {
        let mut resource = ResourceName::new(ResourceType::None);
        resource.entry = String::from("hello");
        resource.package = String::from("com.phodal.rapt");
        assert_eq!("com.phodal.rapt:None/hello", resource.to_string());
    }
}
