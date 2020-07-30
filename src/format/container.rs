use crate::proto::ResourcesInternal::CompiledFile;
use protobuf::{CodedInputStream, CodedOutputStream};
use std::io::Stdin;
use std::net::Shutdown::Write;

static K_CONTAINER_FORMAT_MAGIC: u32 = 0x54504141;
static K_CONTAINER_FORMAT_VERSION: u32 = 1;
static K_PADDING_ALIGNMENT: i32 = 4;

pub struct ContainerWriter<'b> {
    entry_count: i32,
    out: CodedOutputStream<'b>,
}

impl<'b> ContainerWriter<'b> {
    pub fn new<'a>(mut out: CodedOutputStream, entry_count: i32) -> ContainerWriter {
        out.write_raw_little_endian32(K_CONTAINER_FORMAT_MAGIC);
        out.write_raw_little_endian32(K_CONTAINER_FORMAT_VERSION);
        out.write_raw_little_endian32(entry_count.clone() as u32);

        ContainerWriter { entry_count, out }
    }

    pub fn add_res_file_entry(&self, file: CompiledFile, std_in: Stdin) {}
}

#[derive(Clone, Debug)]
pub struct ContainerReader {}

impl ContainerReader {}
