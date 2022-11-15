use crate::{
    color::{Color, Rgba},
    point::Point,
    view::screen::DefaultScreen,
};

pub mod fullblock;
pub mod halfblock;

pub trait Canvas {
    fn init(&mut self, screen: DefaultScreen);
    fn resize(&mut self) -> Point;
    fn clear(&mut self);
    fn draw_pixel(&mut self, p: &Point, rgb: &Rgba);
    fn draw_char(&mut self, p: &Point, color: &Color, ch: char);
    fn draw_text(&mut self, p: &Point, color: &Color, text: &str);
    fn draw_text_transparent(&mut self, p: &Point, fg_color: &Rgba, text: &str);
    fn display(&mut self);
}
