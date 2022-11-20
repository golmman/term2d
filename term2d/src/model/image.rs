use crate::{color::Rgba, point::Point};

#[derive(Clone)]
pub struct Image {
    pub pixels: Vec<Rgba>,
    pub size: Point,
}

impl From<(u32, u32, Vec<u8>)> for Image {
    fn from(raw_image: (u32, u32, Vec<u8>)) -> Self {
        let (width, height, image_bytes) = raw_image;

        let mut image = Image {
            pixels: Vec::new(),
            size: Point::from((width, height)),
        };

        for i in 0..image_bytes.len() / 4 {
            let rgba = Rgba {
                r: image_bytes[4 * i],
                g: image_bytes[4 * i + 1],
                b: image_bytes[4 * i + 2],
                a: image_bytes[4 * i + 3],
            };
            image.pixels.push(rgba);
        }

        image
    }
}

impl Image {
    pub fn mirror_y(&self) -> Self {
        let mut mirrored_image = Self {
            pixels: Vec::new(),
            size: self.size.clone(),
        };

        for y in 0..self.size.height() {
            for x in 0..self.size.width() {
                let index = ((self.size.width() - 1 - x) + y * self.size.width()) as usize;
                let rgba = self.pixels[index].clone();
                mirrored_image.pixels.push(rgba);
            }
        }

        mirrored_image
    }
}
