extern crate image;

pub mod crunch;
pub mod proto;

#[cfg(test)]
mod tests {
    use crate::proto::Resources::ResourceTable;

    #[test]
    fn should_enable_read_file() {
        let table = ResourceTable::new();
    }
}
