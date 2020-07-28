pub mod png;
pub mod png_crunch;
pub mod compile;


#[cfg(test)]
mod tests {
    use crate::crunch::png::Png;
    use std::path::PathBuf;

    #[test]
    fn should_enable_read_file() {
        let buf = PathBuf::from("_fixtures/drawable/image.png");
        let png = Png::read(buf);
        png.to_image(String::from("output.png"));
    }
}
