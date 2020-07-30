use crate::format::archive_writer::IArchiveWriter;
use crate::format::container::ContainerWriter;
use crate::proto::proto_serialize::serialize_compiled_file_to_pb;
use crate::proto::ResourcesInternal::CompiledFile;
use crate::resource::parse_resource_type;
use crate::resource::resource_file::{ResourceFile, ResourceName};
use crate::resource::resource_path_data::ResourcePathData;
use protobuf::CodedOutputStream;

pub struct Compile {}

impl Compile {
    pub fn compile_xml() {}
    pub fn compile_png() {}
    // values
    pub fn compile_table() {}
    pub fn compile_file(path_data: ResourcePathData, output_path: String, writer: IArchiveWriter) {
        let mut res_file = ResourceFile::new();
        let resource_type = parse_resource_type(path_data.resource_dir);
        res_file.name = ResourceName::new(resource_type);
        res_file.source = path_data.source;
        res_file.config = path_data.config;

        write_header_and_data_to_writer(output_path, res_file, Box::from(writer))
    }
}

pub fn write_header_and_data_to_writer(
    output_path: String,
    file: ResourceFile,
    mut writer: Box<IArchiveWriter>,
) {
    writer.start_entry(output_path, 0);

    let output_stream = CodedOutputStream::new(writer.as_mut());
    ContainerWriter::container_writer(output_stream, 1);

    let mut compiled_file = CompiledFile::new();
    serialize_compiled_file_to_pb(file, compiled_file);
}
