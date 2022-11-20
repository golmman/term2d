use crate::{color::Rgba, point::Point};

#[derive(Clone)]
pub struct Image {
    pub size: Point,
    pub pixels: Vec<Rgba>,
}

impl From<(u32, u32, Vec<u8>)> for Image {
    fn from(raw_image: (u32, u32, Vec<u8>)) -> Self {
        let (width, height, image_bytes) = raw_image;

        let mut image = Image {
            size: Point::from((width, height)),
            pixels: Vec::new(),
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
