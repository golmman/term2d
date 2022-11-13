use crate::{
    color::{Color, Rgba},
    point::Point,
    screen::DefaultScreen, rect::Rect,
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

    fn draw_pixel(&mut self, p: Point, rgb: Rgba) {
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
        self.screen
            .as_mut()
            .unwrap()
            .draw_char(scaled_point, color, ch);
    }

    fn draw_text(&mut self, p: Point, color: Color, text: String) {
        let scaled_point = Point::new(p.x, p.y / 2);
        self.screen
            .as_mut()
            .unwrap()
            .draw_text(scaled_point, color, text);
    }

    fn draw_text_transparent(&mut self, p: Point, fg_color: Rgba, text: String) {
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

impl HalfBlockRenderer {
    pub fn draw_rect(&mut self, r: Rect, c: Rgba) {
        let x0 = r.pos.x;
        let x1 = x0 + r.size.width();
        let y0 = r.pos.y;
        let y1 = y0 + r.size.height();

        for y in y0..y1 {
            for x in x0..x1 {
                self.draw_pixel(Point::new(x, y), c.clone());
            }
        }
    }
}
