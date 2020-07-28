use std::path::{PathBuf, Path};
use image::GenericImageView;

pub struct Png {

}

impl Png {
    pub fn read(path: PathBuf) {
        let mut img = image::open(path).unwrap();

        let img_width = img.dimensions().0;
        let img_height = img.dimensions().1;

        for p in img.pixels() {
            println!("pixel: {:?}", p.2);
        }
    }
}
