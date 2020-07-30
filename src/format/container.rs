use protobuf::{CodedInputStream, CodedOutputStream};
use std::net::Shutdown::Write;

static K_CONTAINER_FORMAT_MAGIC: u32 = 0x54504141;
static K_CONTAINER_FORMAT_VERSION: u32 = 1;
static K_PADDING_ALIGNMENT: i32 = 4;

pub struct ContainerWriter<'a> {
    out: CodedOutputStream<'a>,
    entry_count: i32,
}

impl<'a> ContainerWriter<'a> {
    // pub fn new(out: CodedOutputStream, entry_count: i32) -> ContainerWriter<'a> {
    // ContainerWriter { out, entry_count }
    // }
}

#[derive(Clone, Debug)]
pub struct ContainerReader {}

impl ContainerReader {}
