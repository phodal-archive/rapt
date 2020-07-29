use crate::resource::parse_resource_type;
use crate::resource::resource_file::{ResourceFile, ResourceName};
use crate::resource::resource_path_data::ResourcePathData;

pub struct Compile {}

impl Compile {
    pub fn compile_xml() {}
    pub fn compile_png() {}
    // values
    pub fn compile_table() {}
    pub fn compile_file(path_data: ResourcePathData) {
        let mut res_file = ResourceFile::new();
        let resource_type = parse_resource_type(path_data.resource_dir);
        res_file.name = ResourceName::new(resource_type);
        res_file.source = path_data.source;
        res_file.config = path_data.config;
    }
}
