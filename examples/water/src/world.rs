use std::ops::Range;

use term2d::model::image::Image;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;

use crate::random::Random;
use crate::water_rules::setup_water_rules;

pub const RANGE_DIRT: RgbRange = RgbRange::new(66..72, 54..58, 33..35);
pub const RANGE_EMPTY: RgbRange = RgbRange::new(30..40, 30..40, 30..40);
pub const RANGE_WATER_STAY: RgbRange = RgbRange::new(20..30, 40..55, 130..150);
pub const RANGE_WATER_MOVE: RgbRange = RgbRange::new(100..110, 110..125, 200..220);

#[derive(Clone)]
pub struct RgbRange {
    pub r: Range<u8>,
    pub g: Range<u8>,
    pub b: Range<u8>,
}

impl RgbRange {
    const fn new(r: Range<u8>, g: Range<u8>, b: Range<u8>) -> Self {
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

    pub fn get_center(&self) -> Rgba {
        Rgba {
            r: ((self.r.end as u32 + self.r.start as u32) / 2) as u8,
            g: ((self.g.end as u32 + self.g.start as u32) / 2) as u8,
            b: ((self.b.end as u32 + self.b.start as u32) / 2) as u8,
            a: 255,
        }
    }

    pub fn change_in_range(&self, rgba: &Rgba, random: &mut Random) -> Rgba {
        let r = RgbRange::change_in_range_channel(&self.r, rgba.r, random);
        let g = RgbRange::change_in_range_channel(&self.g, rgba.g, random);
        let b = RgbRange::change_in_range_channel(&self.b, rgba.b, random);
        Rgba { r, g, b, a: 255 }
    }

    fn change_in_range_channel(
        range: &Range<u8>,
        channel_intensity: u8,
        random: &mut Random,
    ) -> u8 {
        let new_intensity: i32 = if channel_intensity >= range.end {
            channel_intensity as i32 - 3
        } else if channel_intensity < range.start {
            channel_intensity as i32 + 3
        } else {
            channel_intensity as i32 + ((random.next() % 7) as i32 - 3)
        };

        if new_intensity < 0 {
            0
        } else if new_intensity > 255 {
            255
        } else {
            new_intensity as u8
        }
    }
}

impl Default for RgbRange {
    fn default() -> Self {
        Self {
            r: Default::default(),
            g: Default::default(),
            b: Default::default(),
        }
    }
}

impl From<PixelType> for RgbRange {
    fn from(pixel_type: PixelType) -> Self {
        match pixel_type {
            PixelType::Dirt => RANGE_DIRT,
            PixelType::Empty => RANGE_EMPTY,
            PixelType::Water => RgbRange::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PixelType {
    Dirt,
    Empty,
    Water,
}

pub struct Droplet {
    pub pos: Point,
    pub rgba: Rgba,
    pub rgb_range: RgbRange,
}

impl Droplet {
    pub fn new(p: &Point) -> Self {
        Self {
            pos: p.clone(),
            rgba: RANGE_WATER_STAY.get_center(),
            rgb_range: RANGE_WATER_STAY,
        }
    }

    pub fn copy_move(&self, p: &Point, r: &mut Random) -> Self {
        let rgba = RANGE_WATER_MOVE.change_in_range(&self.rgba, r);
        Self {
            pos: p.clone(),
            rgba,
            rgb_range: RANGE_WATER_MOVE,
        }
    }

    pub fn copy_stay(&self, r: &mut Random) -> Self {
        let rgba = RANGE_WATER_STAY.change_in_range(&self.rgba, r);
        Self {
            pos: self.pos.clone(),
            rgba,
            rgb_range: RANGE_WATER_STAY,
        }
    }
}

pub struct World {
    pub image: Image,
    pub pos: Point,
    pub random: Random,
    pub types: Vec<PixelType>,
    pub water: Vec<Droplet>,
    pub water_rules: Vec<fn(&Droplet, &mut Random) -> Droplet>,
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
            water: Vec::new(),
            water_rules: setup_water_rules(),
        };

        for i in 0..total_pixels as i32 {
            let x = i % size.width();
            let y = i / size.width();

            if x > 3 && x < size.width() - 4 && y < 8 {
                world.set_pixel(&Point::new(x, y), PixelType::Empty);
            } else {
                world.set_pixel(&Point::new(x, y), PixelType::Dirt);
            }
        }

        world
    }

    fn get_index(&self, p: &Point) -> Option<usize> {
        if p.x < 0 || p.x >= self.image.size.width() {
            return None;
        }

        if p.y < 0 || p.y >= self.image.size.height() {
            return None;
        }

        Some((p.x + p.y * self.image.size.width()) as usize)
    }

    pub fn set_pixel(&mut self, p: &Point, pixel_type: PixelType) {
        let Some(index) = self.get_index(p) else {
            return;
        };

        let rgba = RgbRange::from(pixel_type).random_rgb(&mut self.random);

        self.image.pixels[index] = rgba;
        self.types[index] = pixel_type;
    }

    pub fn get_type(&self, p: &Point) -> Option<PixelType> {
        let Some(index) = self.get_index(p) else {
            return None;
        };
        Some(self.types[index])
    }

    pub fn add_droplet(&mut self, p: &Point) {
        let Some(index) = self.get_index(p) else {
            return;
        };

        if self.types[index] != PixelType::Empty {
            return;
        }

        self.water.push(Droplet::new(p));
        self.types[index] = PixelType::Water;
    }

    pub fn simulate_water(&mut self) {
        for i in 0..self.water.len() {
            let Some(old_index) = self.get_index(&self.water[i].pos) else {
                // TODO
                //self.water.remove(i);
                continue;
            };
            self.types[old_index] = PixelType::Empty;

            let neigh = self.get_droplet_neighborhood(&self.water[i]);
            self.water[i] = self.water_rules[neigh](&self.water[i], &mut self.random);

            let Some(new_index) = self.get_index(&self.water[i].pos) else {
                //self.water.remove(i);
                continue;
            };
            self.types[new_index] = PixelType::Water;
        }
    }

    fn get_droplet_neighborhood(&self, d: &Droplet) -> usize {
        let mut neigh = 0;

        if self.get_type(&d.pos.down_right()) != Some(PixelType::Empty) {
            neigh += 1;
        }

        if self.get_type(&d.pos.down()) != Some(PixelType::Empty) {
            neigh += 2;
        }

        if self.get_type(&d.pos.down_left()) != Some(PixelType::Empty) {
            neigh += 4;
        }

        if self.get_type(&d.pos.right()) != Some(PixelType::Empty) {
            neigh += 8;
        }

        if self.get_type(&d.pos.left()) != Some(PixelType::Empty) {
            neigh += 16;
        }

        neigh
    }
}
