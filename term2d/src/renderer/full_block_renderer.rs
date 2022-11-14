use crate::screen::DefaultScreen;

use super::Renderer;

pub struct FullblockCanvas {
    screen: Option<DefaultScreen>,
}

impl FullblockCanvas {
    pub fn new() -> Self {
        Self { screen: None }
    }
}

impl Renderer for FullblockCanvas {
    fn init(&mut self, screen: DefaultScreen) {
        self.screen = Some(screen);
    }

    fn resize(&mut self) -> crate::point::Point {
        self.screen.as_mut().unwrap().resize()
    }

    fn clear(&mut self) {
        self.screen.as_mut().unwrap().clear();
    }

    fn draw_pixel(&mut self, p: crate::point::Point, rgb: crate::color::Rgba) {
        self.screen.as_mut().unwrap().draw_pixel(p, rgb);
    }

    fn draw_char(&mut self, p: crate::point::Point, color: crate::color::Color, ch: char) {
        self.screen.as_mut().unwrap().draw_char(p, color, ch);
    }

    fn draw_text(&mut self, p: crate::point::Point, color: crate::color::Color, text: String) {
        self.screen.as_mut().unwrap().draw_text(p, color, text);
    }

    fn draw_text_transparent(
        &mut self,
        p: crate::point::Point,
        fg_color: crate::color::Rgba,
        text: String,
    ) {
        self.screen
            .as_mut()
            .unwrap()
            .draw_text_transparent(p, fg_color, text);
    }

    fn display(&mut self) {
        self.screen.as_mut().unwrap().display();
    }
}
