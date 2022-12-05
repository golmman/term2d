use crate::model::color::Color;
use crate::model::point::Point;
use crate::model::rgba::Rgba;
use crate::view::screen::DefaultScreen;

use super::Canvas;

const HALF_BLOCK: char = '▀';

pub struct HalfblockCanvas {
    screen: Option<DefaultScreen>,
}

impl HalfblockCanvas {
    pub fn new() -> Self {
        Self { screen: None }
    }
}

impl From<DefaultScreen> for HalfblockCanvas {
    fn from(screen: DefaultScreen) -> Self {
        Self {
            screen: Some(screen),
        }
    }
}

impl Canvas for HalfblockCanvas {
    fn init(&mut self, screen: DefaultScreen) {
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
        let x = p.x;
        let y = p.y / 2;

        let old_color = self.screen.as_ref().unwrap().get_color(&Point::new(x, y));

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

        self.screen
            .as_mut()
            .unwrap()
            .draw_char(&Point::new(x, y), &new_color, HALF_BLOCK);
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
