use crate::{
    color::{Color, Rgb},
    point::Point,
    screen::DefaultScreen,
};

pub mod full_block_renderer;
pub mod half_block_renderer;

pub trait Renderer {
    fn init(&mut self, screen: DefaultScreen);
    fn resize(&mut self) -> Point;
    fn clear(&mut self);
    fn draw_pixel(&mut self, p: Point, rgb: Rgb);
    fn draw_char(&mut self, p: Point, color: Color, ch: char);
    fn draw_text(&mut self, p: Point, color: Color, text: String);
    fn draw_text_transparent(&mut self, p: Point, fg_color: Rgb, text: String);
    fn display(&mut self);
}
