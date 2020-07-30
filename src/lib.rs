extern crate image;

pub mod androidrw;
pub mod compile;
pub mod crunch;
pub mod format;
pub mod proto;
pub mod resource;

#[cfg(test)]
mod tests {
    use crate::proto::Resources::ResourceTable;

    #[test]
    fn should_enable_read_file() {
        let _table = ResourceTable::new();
    }
}
