use crate::proto::ResourcesInternal::CompiledFile;
use crate::resource::resource_file::ResourceFile;

pub fn serialize_compiled_file_to_pb(file: ResourceFile, mut out_file: CompiledFile) {
    out_file.set_resource_name(file.name.to_string());
}
