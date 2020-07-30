use crate::proto::ResourcesInternal::CompiledFile;
use protobuf::{CodedInputStream, CodedOutputStream};
use std::io::Stdin;
use std::net::Shutdown::Write;

static K_CONTAINER_FORMAT_MAGIC: u32 = 0x54504141;
static K_CONTAINER_FORMAT_VERSION: u32 = 1;
static K_PADDING_ALIGNMENT: i32 = 4;

pub struct ContainerWriter {
    entry_count: i32,
}

impl ContainerWriter {
    pub fn container_writer(mut out: CodedOutputStream, entry_count: i32) {
        out.write_raw_little_endian32(K_CONTAINER_FORMAT_MAGIC);
        out.write_raw_little_endian32(K_CONTAINER_FORMAT_VERSION);
        out.write_raw_little_endian32(entry_count as u32);
    }

    pub fn add_res_file_entry(file: CompiledFile, std_in: Stdin) {}
}

#[derive(Clone, Debug)]
pub struct ContainerReader {}

impl ContainerReader {}
