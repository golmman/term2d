use std::ops::Range;

use term2d::{color::Rgba, model::image::Image, point::Point};

use crate::random::Random;

struct RgbRange {
    pub r: Range<u8>,
    pub g: Range<u8>,
    pub b: Range<u8>,
}

impl RgbRange {
    fn new(r: Range<u8>, g: Range<u8>, b: Range<u8>) -> Self {
        Self { r, g, b }
    }

    pub fn random_rgb(&self, random: &mut Random) -> Rgba {
        Rgba {
            r: random.next_range_u8(&self.r),
            g: random.next_range_u8(&self.g),
            b: random.next_range_u8(&self.b),
            a: 255,
        }
    }
}

impl From<PixelType> for RgbRange {
    fn from(pixel_type: PixelType) -> Self {
        match pixel_type {
            PixelType::Dirt => RgbRange::new(66..72, 54..58, 33..35),
            PixelType::Empty => RgbRange::new(0..0, 0..0, 0..0),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PixelType {
    Dirt,
    Empty,
}

pub struct World {
    pub image: Image,
    pub pos: Point,
    pub random: Random,
    pub types: Vec<PixelType>,
}

impl World {
    pub fn new(pos: &Point, size: &Point) -> Self {
        let random = Random::new();
        let total_pixels = (size.width() * size.height()) as usize;

        let image = Image {
            pixels: vec![Rgba::default(); total_pixels],
            size: size.clone(),
        };

        let mut world = Self {
            image,
            pos: pos.clone(),
            random,
            types: vec![PixelType::Empty; total_pixels],
        };

        for i in 0..total_pixels as i32 {
            let x = i % size.width();
            let y = i / size.width();

            if x > 3 && x < size.width() - 4 && y < 3 {
                world.set_pixel(&Point::new(x, y), PixelType::Empty);
            } else {
                world.set_pixel(&Point::new(x, y), PixelType::Dirt);
            }
        }

        world
    }

    pub fn set_pixel(&mut self, p: &Point, pixel_type: PixelType) {
        let rgba = RgbRange::from(pixel_type).random_rgb(&mut self.random);
        let index = (p.x + p.y * self.image.size.width()) as usize;

        self.image.pixels[index] = rgba;
        self.types[index] = pixel_type;
    }

    pub fn get_type(&mut self, p: &Point) -> PixelType {
        let index = (p.x + p.y * self.image.size.width()) as usize;
        self.types[index]
    }
}
