use super::Canvas;
use crate::model::color::Color;
use crate::model::point::Point;
use crate::model::rect::Rect;
use crate::model::rgba::Rgba;
use crate::view::screen::RawTerminalScreen;
use crate::view::screen::Screen;

const HALF_BLOCK: char = '▀';

pub struct HalfblockCanvas {
    screen: Option<RawTerminalScreen>,
    size: Point,
}

impl HalfblockCanvas {
    pub fn new() -> Self {
        Self {
            screen: None,
            size: Point::new(0, 0),
        }
    }
}

impl Canvas for HalfblockCanvas {
    fn init(&mut self, screen: RawTerminalScreen) {
        let screen_size = screen.get_size();
        self.size = Point::new(screen_size.width(), 2 * screen_size.height());
        self.screen = Some(screen);
    }

    fn get_size(&self) -> &Point {
        &self.size
    }

    fn resize(&mut self) -> &Point {
        let screen_size = self.screen.as_mut().unwrap().resize();
        self.size = Point::new(screen_size.width(), 2 * screen_size.height());
        &self.size
    }

    fn clear(&mut self) {
        self.screen.as_mut().unwrap().clear();
    }

    fn draw_pixel(&mut self, p: &Point, rgb: &Rgba) {
        let screen = self.screen.as_mut().unwrap();
        let x = p.x;
        let y = p.y / 2;

        if !Rect::from(&self.size).contains(p) {
            return;
        }

        let old_color = screen.get_color(&Point::new(x, y));

        let new_color = if p.y % 2 == 0 {
            Color {
                bg: old_color.bg,
                fg: rgb.clone(),
            }
        } else {
            Color {
                bg: rgb.clone(),
                fg: old_color.fg,
            }
        };

        screen.draw_char(&Point::new(x, y), &new_color, HALF_BLOCK);
    }

    fn draw_char(&mut self, p: &Point, color: &Color, ch: char) {
        let scaled_point = &Point::new(p.x, p.y / 2);
        self.screen
            .as_mut()
            .unwrap()
            .draw_char(&scaled_point, color, ch);
    }

    fn draw_text(&mut self, p: &Point, color: &Color, text: &str) {
        let scaled_point = &Point::new(p.x, p.y / 2);
        self.screen
            .as_mut()
            .unwrap()
            .draw_text(scaled_point, color, text);
    }

    fn display(&mut self) {
        self.screen.as_mut().unwrap().display();
    }
}
