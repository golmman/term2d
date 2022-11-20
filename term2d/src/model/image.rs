use crate::{color::Rgba, point::Point};

pub struct Image {
    pub size: Point,
    pub pixels: Vec<Rgba>,
}

pub struct Video {
    pub frame: usize,
    pub images: Vec<Image>,
}

impl From<Vec<(u32, u32, Vec<u8>)>> for Video {
    fn from(raw_video: Vec<(u32, u32, Vec<u8>)>) -> Self {
        let mut video = Self {
            frame: 0,
            images: Vec::new(),
        };

        for frame in raw_video {
            let (width, height, image_bytes) = frame;
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

            video.images.push(image);
        }

        video
    }
}
