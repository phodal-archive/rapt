pub struct IArchiveWriter {
    path: String,
    flags: u32,
    uncompressed_size: i64,
}

impl IArchiveWriter {
    pub fn write_file(&self) {}
    pub fn start_entry(&self, output_path: String, flags: usize) {}
}
