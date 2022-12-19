use crate::model::color::Color;
use crate::model::point::Point;
use crate::model::rect::Rect;
use crate::model::rgba::Rgba;
use crate::view::screen::DefaultScreen;

use super::Canvas;

pub struct FullblockCanvas {
    screen: Option<DefaultScreen>,
}

impl FullblockCanvas {
    pub fn new() -> Self {
        Self { screen: None }
    }
}

impl From<DefaultScreen> for FullblockCanvas {
    fn from(screen: DefaultScreen) -> Self {
        Self {
            screen: Some(screen),
        }
    }
}

impl Canvas for FullblockCanvas {
    fn init(&mut self, screen: DefaultScreen) {
        self.screen = Some(screen);
    }

    fn resize(&mut self) -> Point {
        self.screen.as_mut().unwrap().resize()
    }

    fn clear(&mut self) {
        self.screen.as_mut().unwrap().clear();
    }

    fn draw_pixel(&mut self, p: &Point, rgb: &Rgba) {
        let screen = self.screen.as_mut().unwrap();

        if !Rect::from(&screen.size).contains(p) {
            return;
        }

        screen.draw_pixel(p, rgb);
    }

    fn draw_char(&mut self, p: &Point, color: &Color, ch: char) {
        self.screen.as_mut().unwrap().draw_char(p, color, ch);
    }

    fn draw_text(&mut self, p: &Point, color: &Color, text: &str) {
        self.screen.as_mut().unwrap().draw_text(p, color, text);
    }

    fn display(&mut self) {
        self.screen.as_mut().unwrap().display();
    }
}
