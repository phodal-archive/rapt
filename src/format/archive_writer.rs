use std::io::Write;

pub struct IArchiveWriter {
    path: String,
    flags: u32,
    uncompressed_size: i64,
}

impl Write for IArchiveWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(1)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl IArchiveWriter {
    pub fn write_file(&self) {}
    pub fn start_entry(&self, output_path: String, flags: usize) {}
}
