use crate::format::archive_writer::IArchiveWriter;
use crate::format::container::ContainerWriter;
use crate::proto::proto_serialize::serialize_compiled_file_to_pb;
use crate::proto::ResourcesInternal::CompiledFile;
use crate::resource::parse_resource_type;
use crate::resource::resource_file::{ResourceFile, ResourceFileType, ResourceName};
use crate::resource::resource_path_data::ResourcePathData;
use crate::xml::xml_dom::{inflate, XmlResource};
use protobuf::CodedOutputStream;
use std::io;

pub struct Compile {}

impl Compile {
    pub fn compile_xml(path_data: ResourcePathData) {
        let mut xml_file = ResourceFile::new();
        let mut xml_resource = XmlResource::new(xml_file.clone());
        xml_resource = inflate(xml_file);

        let resource_type = parse_resource_type(path_data.resource_dir);
        xml_file.name = ResourceName::new(resource_type);
        xml_file.source = path_data.source;
        xml_file.config = path_data.config;
        xml_file.typ = ResourceFileType::kProtoXml;
    }
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
    let mut container_writer = ContainerWriter::new(output_stream, 1);

    let pb_compiled_file = CompiledFile::new();
    serialize_compiled_file_to_pb(file, pb_compiled_file.clone());

    let stdin = io::stdin();
    container_writer.add_res_file_entry(pb_compiled_file, stdin);
}
