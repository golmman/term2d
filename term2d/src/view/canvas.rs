use crate::model::circle::Circle;
use crate::model::color::Color;
use crate::model::image::Image;
use crate::model::point::Point;
use crate::model::polygon::Polygon;
use crate::model::rect::Rect;
use crate::model::rgba::Rgba;
use crate::model::video::Video;

use super::screen::RawTerminalScreen;

pub mod fullblock;
pub mod halfblock;

pub trait Canvas: Sized {
    fn init(&mut self, screen: RawTerminalScreen);
    fn get_size(&self) -> &Point;
    fn resize(&mut self) -> &Point;
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

        for x in x0..x1 {
            self.draw_pixel(&Point::new(x, y0), c);
            self.draw_pixel(&Point::new(x, y1 - 1), c);
        }

        for y in y0..y1 {
            self.draw_pixel(&Point::new(x0, y), c);
            self.draw_pixel(&Point::new(x1 - 1, y), c);
        }
    }

    fn draw_rect_fill(&mut self, r: &Rect, c: &Rgba) {
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

    fn draw_circle(&mut self, circle: &Circle, rgba: &Rgba) {
        let cx = circle.pos.x;
        let cy = circle.pos.y;
        let mut x = circle.radius;
        let mut y = 0;
        let mut decision_over_2 = 1 - x;

        while y <= x {
            self.draw_pixel(&Point::new(cx + x, cy + y), rgba);
            self.draw_pixel(&Point::new(cx + y, cy + x), rgba);
            self.draw_pixel(&Point::new(cx - y, cy + x), rgba);
            self.draw_pixel(&Point::new(cx - x, cy + y), rgba);
            self.draw_pixel(&Point::new(cx - x, cy - y), rgba);
            self.draw_pixel(&Point::new(cx - y, cy - x), rgba);
            self.draw_pixel(&Point::new(cx + y, cy - x), rgba);
            self.draw_pixel(&Point::new(cx + x, cy - y), rgba);
            y += 1;
            if decision_over_2 <= 0 {
                decision_over_2 += 2 * y + 1;
            } else {
                x -= 1;
                decision_over_2 += 2 * (y - x) + 1;
            }
        }
    }

    fn draw_circle_fill(&mut self, circle: &Circle, rgba: &Rgba) {
        let cx = circle.pos.x;
        let cy = circle.pos.y;
        let radius = circle.radius;
        let min_x = cx - radius;
        let max_x = cx + radius;
        let min_y = cy - radius;
        let max_y = cy + radius;

        for x in min_x..max_x {
            for y in min_y..max_y {
                let dx = (x as i32 - cx as i32).abs();
                let dy = (y as i32 - cy as i32).abs();
                let distance = dx * dx + dy * dy;

                if distance < radius * radius {
                    self.draw_pixel(&Point::new(x, y), rgba);
                }
            }
        }
    }

    fn draw_polygon(&mut self, polygon: &Polygon, rgba: &Rgba) {
        let vertices = polygon.vertices();
        for i in 0..vertices.len() - 1 {
            self.draw_line(&vertices[i], &vertices[i + 1], rgba);
        }
        self.draw_line(&vertices[vertices.len() - 1], &vertices[0], rgba);
    }

    fn draw_polygon_fill(&mut self, polygon: &Polygon, rgba: &Rgba) {
        let boundary = polygon.boundary();
        let min_x = boundary.pos.x;
        let max_x = min_x + boundary.size.width();
        let min_y = boundary.pos.y;
        let max_y = min_y + boundary.size.height();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let p = &Point::new(x, y);
                if polygon.is_inside(p) {
                    self.draw_pixel(p, rgba);
                }
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

    fn draw_video(&mut self, p: &Point, video: &Video) {
        let image = &video.images[video.frame];
        self.draw_image(p, image);
    }
}
