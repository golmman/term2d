use crate::model::color::Color;
use crate::model::point::Point;
use crate::model::rect::Rect;
use crate::model::rgba::Rgba;
use crate::view::screen::{DefaultScreen, RawTerminalScreen};
use crate::view::screen::Screen2;
use super::Canvas;

const HALF_BLOCK: char = '▀';

pub struct HalfblockCanvas {
    screen: Option<RawTerminalScreen>,
}

impl HalfblockCanvas {
    pub fn new() -> Self {
        Self { screen: None }
    }
}

impl From<RawTerminalScreen> for HalfblockCanvas {
    fn from(screen: RawTerminalScreen) -> Self {
        Self {
            screen: Some(screen),
        }
    }
}

impl Canvas for HalfblockCanvas {
    fn init(&mut self, screen: RawTerminalScreen) {
        self.screen = Some(screen);
    }

    fn resize(&mut self) -> Point {
        let screen_size = self.screen.as_mut().unwrap().resize();
        Point::new(screen_size.width(), screen_size.height() * 2)
    }

    fn clear(&mut self) {
        self.screen.as_mut().unwrap().clear();
    }

    fn draw_pixel(&mut self, p: &Point, rgb: &Rgba) {
        let screen = self.screen.as_mut().unwrap();
        let screen_size = Point::new(screen.size.width(), 2 * screen.size.height());
        let x = p.x;
        let y = p.y / 2;

        if !Rect::from(&screen_size).contains(p) {
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
