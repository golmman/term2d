use super::Canvas;
use crate::model::color::Color;
use crate::model::point::Point;
use crate::model::rect::Rect;
use crate::model::rgba::Rgba;
use crate::view::screen::RawTerminalScreen;
use crate::view::screen::Screen;

pub struct FullblockCanvas {
    screen: Option<RawTerminalScreen>,
    size: Point,
}

impl FullblockCanvas {
    pub fn new() -> Self {
        Self {
            screen: None,
            size: Point::new(0, 0),
        }
    }
}

impl Canvas for FullblockCanvas {
    fn init(&mut self, screen: RawTerminalScreen) {
        self.size = screen.get_size().clone();
        self.screen = Some(screen);
    }

    fn get_size(&self) -> &Point {
        &self.size
    }

    fn resize(&mut self) -> &Point {
        self.size = self.screen.as_mut().unwrap().resize().clone();
        &self.size
    }

    fn clear(&mut self) {
        self.screen.as_mut().unwrap().clear();
    }

    fn draw_pixel(&mut self, p: &Point, rgb: &Rgba) {
        let screen = self.screen.as_mut().unwrap();

        if !Rect::from(&self.size).contains(p) {
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
