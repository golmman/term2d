use crate::{
    color::{Color, Rgb},
    point::Point,
    screen::DefaultScreen,
};

use super::Renderer;

const HALF_BLOCK: char = '▀';

pub struct HalfBlockRenderer {
    screen: Option<DefaultScreen>,
}

impl HalfBlockRenderer {
    pub fn new() -> Self {
        Self { screen: None }
    }
}

impl Renderer for HalfBlockRenderer {
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

    fn draw_pixel(&mut self, p: Point, rgb: Rgb) {
        let x = p.x;
        let y = p.y / 2;

        let old_color = self.screen.as_ref().unwrap().get_color(Point::new(x, y));

        let new_color = if p.y % 2 == 0 {
            Color {
                bg: old_color.bg,
                fg: rgb,
            }
        } else {
            Color {
                bg: rgb,
                fg: old_color.fg,
            }
        };

        self.screen
            .as_mut()
            .unwrap()
            .draw_char(Point::new(x, y), new_color, HALF_BLOCK);
    }

    fn draw_char(&mut self, p: Point, color: Color, ch: char) {
        let scaled_point = Point::new(p.x, p.y / 2);
        self.screen.as_mut().unwrap().draw_char(scaled_point, color, ch);
    }

    fn draw_text(&mut self, p: Point, color: Color, text: String) {
        let scaled_point = Point::new(p.x, p.y / 2);
        self.screen.as_mut().unwrap().draw_text(scaled_point, color, text);
    }

    fn draw_text_transparent(&mut self, p: Point, fg_color: Rgb, text: String) {
        let scaled_point = Point::new(p.x, p.y / 2);
        self.screen
            .as_mut()
            .unwrap()
            .draw_text_transparent(scaled_point, fg_color, text);
    }

    fn display(&mut self) {
        self.screen.as_mut().unwrap().display();
    }
}
