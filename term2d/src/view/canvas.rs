use crate::model::color::Color;
use crate::model::image::Image;
use crate::model::point::Point;
use crate::model::rect::Rect;
use crate::model::rgba::Rgba;
use crate::model::video::Video;
use crate::view::screen::DefaultScreen;

pub mod fullblock;
pub mod halfblock;

pub trait Canvas: Sized {
    fn init(&mut self, screen: DefaultScreen);
    fn resize(&mut self) -> Point;
    fn clear(&mut self);
    fn draw_pixel(&mut self, p: &Point, rgb: &Rgba);
    fn draw_char(&mut self, p: &Point, color: &Color, ch: char);
    fn draw_text(&mut self, p: &Point, color: &Color, text: &str);
    fn display(&mut self);

    fn draw_rect(&mut self, r: &Rect, c: &Rgba) {
        let x0 = r.pos.x;
        let x1 = x0 + r.size.width();
        let y0 = r.pos.y;
        let y1 = y0 + r.size.height();

        for y in y0..y1 {
            for x in x0..x1 {
                self.draw_pixel(&Point::new(x, y), c);
            }
        }
    }

    fn draw_line(&mut self, p1: &Point, p2: &Point, c: &Rgba) {
        let mut x = p1.x;
        let mut y = p1.y;
        let dx = (p2.x - p1.x).abs();
        let dy = (p2.y - p1.y).abs();
        let sx = if p1.x < p2.x { 1 } else { -1 };
        let sy = if p1.y < p2.y { 1 } else { -1 };
        let mut err = if dx > dy { dx } else { -dy } / 2;

        loop {
            self.draw_pixel(&Point::new(x, y), c);

            if x == p2.x && y == p2.y {
                break;
            }

            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x += sx;
            }
            if e2 < dy {
                err += dx;
                y += sy;
            }
        }
    }

    fn draw_image(&mut self, p: &Point, image: &Image) {
        for y in 0..image.size.height() {
            for x in 0..image.size.width() {
                let index = (x + y * image.size.width()) as usize;
                let rgba = &image.pixels[index];
                self.draw_pixel(&Point::new(p.x + x, p.y + y), rgba);
            }
        }
    }

    fn draw_video(&mut self, p: &Point, video: &mut Video) {
        let image = &video.images[video.frame];
        self.draw_image(p, image);
        video.frame = (video.frame + 1) % video.images.len();
    }
}
