use crate::proto::Configuration::Configuration;
use crate::proto::Resources::FileReference_Type;
use crate::proto::ResourcesInternal::{CompiledFile, CompiledFile_Symbol};
use crate::resource::config_description::ConfigDescription;
use crate::resource::resource_file::{ResourceFile, ResourceFileType};

pub fn serialize_compiled_file_to_pb(file: ResourceFile, mut out_file: CompiledFile) {
    out_file.set_resource_name(file.name.to_string());
    out_file.set_source_path(file.source.path);
    out_file.set_field_type(serialize_file_reference_type_to_pb(file.typ));

    serialize_config(file.config, out_file.mut_config());

    for exported in file.exported_symbols {
        let mut pb_symbol = CompiledFile_Symbol::new();
        pb_symbol.set_resource_name(exported.name.to_string());
        pb_symbol.mut_source().set_line_number(exported.line as u32);

        out_file.exported_symbol.push(pb_symbol);
    }
}

pub fn serialize_file_reference_type_to_pb(typ: ResourceFileType) -> FileReference_Type {
    match typ {
        ResourceFileType::kPng => FileReference_Type::PNG,
        ResourceFileType::kBinaryXml => FileReference_Type::BINARY_XML,
        ResourceFileType::kProtoXml => FileReference_Type::PROTO_XML,
        _ => FileReference_Type::UNKNOWN,
    }
}

pub fn serialize_config(_config: ConfigDescription, _out_pb_config: &mut Configuration) {
    // out_pb_config.set_mcc(config.mcc);
    // out_pb_config.set_mnc(config.mnc);
    // out_pb_config.set_locale(config.GetBcp47LanguageTag());
}
