use std::ops::Range;

use term2d::{color::Rgba, model::image::Image, point::Point};

use crate::random::Random;

// 69, 56, 34
const BROWN: RgbRange = RgbRange {
    r: 66..72,
    g: 54..58,
    b: 33..35,
};

struct RgbRange {
    pub r: Range<u8>,
    pub g: Range<u8>,
    pub b: Range<u8>,
}

impl RgbRange {
    pub fn random_rgb(&self, random: &mut Random) -> Rgba {
        Rgba {
            r: random.next_range_u8(&self.r),
            g: random.next_range_u8(&self.g),
            b: random.next_range_u8(&self.b),
            a: 255,
        }
    }
}

pub struct World {
    pub image: Image,
    pub random: Random,
}

impl World {
    pub fn new(size: &Point) -> Self {
        let mut random = Random::new();

        let mut pixels = Vec::new();

        for _i in 0..size.width() * size.height() {
            pixels.push(BROWN.random_rgb(&mut random));
        }

        let image = Image {
            pixels,
            size: size.clone(),
        };

        Self { image, random }
    }
}
