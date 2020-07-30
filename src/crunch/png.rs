use image::GenericImageView;
use std::path::PathBuf;

pub enum ColorType {
    PNG_COLOR_TYPE_GRAY,
    PNG_COLOR_TYPE_GRAY_ALPHA,
    PNG_COLOR_TYPE_PALETTE,
    PNG_COLOR_TYPE_RGB,
    PNG_COLOR_TYPE_RGB_ALPHA,
    PNG_COLOR_MASK_PALETTE,
    PNG_COLOR_MASK_COLOR,
    PNG_COLOR_MASK_ALPHA,
}

pub enum FilterMethod {
    PNG_FILTER_TYPE_BASE,
    PNG_INTRAPIXEL_DIFFERENCING,
}

pub enum InterlaceMethod {
    PNG_INTERLACE_NONE,
    PNG_INTERLACE_ADAM7,
}

#[derive(Debug, Clone)]
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
