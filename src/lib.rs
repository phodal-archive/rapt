extern crate image;

pub mod androidrw;
pub mod compile;
pub mod compiler;
pub mod crunch;
pub mod format;
pub mod process;
pub mod proto;
pub mod resource;
pub mod xml;

#[cfg(test)]
mod tests {
    use crate::proto::Resources::ResourceTable;

    #[test]
    fn should_enable_read_file() {
        let _table = ResourceTable::new();
    }
}
