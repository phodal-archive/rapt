use crate::resource::config_description::ConfigDescription;
use crate::resource::source::Source;

pub struct ResourcePathData {
    pub(crate) source: Source,
    pub(crate) resource_dir: String,
    pub(crate) name: String,
    pub(crate) extension: String,
    pub(crate) config_str: String,
    pub(crate) config: ConfigDescription,
}

impl ResourcePathData {
    pub fn extract() {}
}
