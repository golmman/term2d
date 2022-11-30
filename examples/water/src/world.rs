use std::{
    cmp::{max, min},
    ops::Range,
};

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
            PixelType::Empty => RgbRange::new(30..40, 30..40, 30..40),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PixelType {
    Dirt,
    Empty,
}

const DROPLET_SCALE: i32 = 100;

pub struct Droplet {
    pub pos: Point, // centi
    pub vel: Point,
}

impl Droplet {
    pub fn new(p: &Point) -> Self {
        Self {
            pos: Point::new(p.x * DROPLET_SCALE, p.y * DROPLET_SCALE),
            vel: Point::new(0, 0),
        }
    }

    pub fn get_pos(&self) -> Point {
        Point::new(self.pos.x / DROPLET_SCALE, self.pos.y / DROPLET_SCALE)
    }

    pub fn limit_vel(&mut self) {
        //if self.vel.x < -DROPLET_SCALE {
        //    self.vel.x = -DROPLET_SCALE;
        //}
        //if self.vel.x > DROPLET_SCALE {
        //    self.vel.x = DROPLET_SCALE;
        //}
        //if self.vel.y < -DROPLET_SCALE {
        //    self.vel.y = -DROPLET_SCALE;
        //}
        //if self.vel.y > DROPLET_SCALE {
        //    self.vel.y = DROPLET_SCALE;
        //}

        let vel = ((self.vel.x * self.vel.x + self.vel.y * self.vel.y) as f32).sqrt();
        if vel > DROPLET_SCALE as f32 {
            self.vel.x = ((DROPLET_SCALE * self.vel.x) as f32 / vel) as i32;
            self.vel.y = ((DROPLET_SCALE * self.vel.y) as f32 / vel) as i32;
        }
    }
}

pub struct World {
    pub image: Image,
    pub pos: Point,
    pub random: Random,
    pub types: Vec<PixelType>,
    pub water: Vec<Droplet>,
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

    pub fn simulate_water(&mut self) {
        for i in 0..self.water.len() {
            if let Some(droplet) = self.simulate_droplet_collisions(&self.water[i]) {
                self.water[i] = droplet;
                continue;
            }

            self.water[i].pos.x += self.water[i].vel.x;
            self.water[i].pos.y += self.water[i].vel.y;

            self.water[i].vel.y += 10;
            self.water[i].limit_vel();
        }
    }

    pub fn simulate_droplet_collisions(&self, droplet: &Droplet) -> Option<Droplet> {
        None
    }
}
