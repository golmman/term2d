use crate::{
    color::{Color, Rgba},
    model::image::Video,
    point::Point,
    rect::Rect,
    view::screen::DefaultScreen,
};

pub mod fullblock;
pub mod halfblock;

pub trait Canvas: Sized {
    fn init(&mut self, screen: DefaultScreen);
    fn resize(&mut self) -> Point;
    fn clear(&mut self);
    fn draw_pixel(&mut self, p: &Point, rgb: &Rgba);
    fn draw_char(&mut self, p: &Point, color: &Color, ch: char);
    fn draw_text(&mut self, p: &Point, color: &Color, text: &str);
    fn display(&mut self);

    fn draw_rect(&mut self, r: &Rect, c: &Rgba) {
        let x0 = r.pos.x;
        let x1 = x0 + r.size.width();
        let y0 = r.pos.y;
        let y1 = y0 + r.size.height();

        for y in y0..y1 {
            for x in x0..x1 {
                self.draw_pixel(&Point::new(x, y), c);
            }
        }
    }

    fn draw_video(&mut self, p: &Point, v: &mut Video) {
        let image = &v.images[v.frame];

        for y in 0..image.size.height() {
            for x in 0..image.size.width() {
                let index = (x + y * image.size.width()) as usize;
                let rgb = &image.pixels[index];
                self.draw_pixel(&Point::new(p.x + x, p.y + y), rgb);
            }
        }

        v.frame = (v.frame + 1) % v.images.len();
    }
}
