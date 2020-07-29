extern crate image;

pub mod compile;
pub mod crunch;
pub mod proto;
pub mod resource_file;
pub mod source;

#[cfg(test)]
mod tests {
    use crate::proto::Resources::ResourceTable;

    #[test]
    fn should_enable_read_file() {
        let table = ResourceTable::new();
    }
}
