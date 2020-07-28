use std::path::{PathBuf};
use image::GenericImageView;

pub struct Png {
    width: u32,
    height: u32,
}

impl Png {
    pub fn new(width: u32, height: u32) -> Png {
        Png { width, height }
    }

    pub fn read(path: PathBuf) -> Png {
        let mut img = image::open(path).unwrap();

        let width = img.dimensions().0;
        let height = img.dimensions().1;

        for (x, y, pixel) in img.pixels() {
            println!("pixel: {:?}", pixel);
        }
        Png::new(width, height)
    }

    pub fn to_image(&self, path: String) {
        let width = self.width.clone();
        let height = self.height.clone();
        let mut imgbuf = image::ImageBuffer::new(width, height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let r = (0.3 * x as f32) as u8;
            let b = (0.3 * y as f32) as u8;
            *pixel = image::Rgb([r, 0, b]);
        }

        imgbuf.save(path).unwrap();
    }
}
