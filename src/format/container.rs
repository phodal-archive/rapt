use crate::proto::ResourcesInternal::CompiledFile;
use protobuf::{CodedInputStream, CodedOutputStream, Message};
use std::io::Stdin;
use std::net::Shutdown::Write;

static K_CONTAINER_FORMAT_MAGIC: u32 = 0x54504141;
static K_CONTAINER_FORMAT_VERSION: u32 = 1;
static K_PADDING_ALIGNMENT: i32 = 4;

pub struct ContainerWriter<'b> {
    entry_count: i32,
    out: CodedOutputStream<'b>,
}

static kResTable: u8 = 0x00;
static kResFile: u8 = 0x01;

impl<'b> ContainerWriter<'b> {
    pub fn new<'a>(mut stream: CodedOutputStream, entry_count: i32) -> ContainerWriter {
        let mut out = CodedOutputStream::from(stream);

        out.write_raw_little_endian32(K_CONTAINER_FORMAT_MAGIC);
        out.write_raw_little_endian32(K_CONTAINER_FORMAT_VERSION);
        out.write_raw_little_endian32(entry_count.clone() as u32);

        ContainerWriter { entry_count, out }
    }

    pub fn add_res_file_entry(&mut self, file: CompiledFile, std_in: Stdin) {
        self.out.write_raw_little_endian32(kResFile as u32);

        let header_size = file.compute_size();
        let header_padding = 4 - (header_size % 4);
    }
}

#[derive(Clone, Debug)]
pub struct ContainerReader {}

impl ContainerReader {}
