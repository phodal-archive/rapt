pub mod png;
pub mod compile;


#[cfg(test)]
mod tests {
    use crate::crunch::png::Png;
    use std::path::PathBuf;

    #[test]
    fn should_enable_read_file() {
        let buf = PathBuf::from("_fixtures/drawable/image.png");
        Png::read(buf);
    }
}
