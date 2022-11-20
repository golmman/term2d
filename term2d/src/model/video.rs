use super::image::Image;

#[derive(Clone)]
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
            let image = Image::from(frame);
            video.images.push(image);
        }

        video
    }
}
