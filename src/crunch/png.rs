use std::path::{PathBuf, Path};
use image::GenericImageView;

pub struct Png {
    width: u32,
    height: u32
}

impl Png {
    pub fn new(width: u32, height: u32) -> Png {
        Png { width, height }
    }

    pub fn read(path: PathBuf) -> Png {
        let mut img = image::open(path).unwrap();

        let width = img.dimensions().0;
        let height = img.dimensions().1;

        for p in img.pixels() {
            println!("pixel: {:?}", p.2);
        }
        Png::new(width, height)
    }
}
