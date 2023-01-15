use term2d::model::color::Color;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;

use crate::model::state::State;

pub struct Renderer {
    pub canvas: HalfblockCanvas,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            canvas: HalfblockCanvas::new(),
        }
    }

    pub fn draw(&mut self, state: &State) {
        self.canvas.clear();
        self.canvas.draw_text(
            &Point::new(2, 0),
            &Color {
                fg: Rgba::white(),
                bg: Rgba::transparent(),
            },
            &format!("press 'q' to quit, frame: {}", state.frame),
        );
        self.canvas.draw_pixel(&Point::new(10, 7), &Rgba::red());

        self.draw_tetromino(&state);

        self.canvas.display();
    }

    fn draw_tetromino(&mut self, state: &State) {
        for y in 0..4 {
            for x in 0..4 {
                if state.tetromino.is_pixel_visible(x, y) {
                    self.canvas
                        .draw_pixel(&Point::new(x, y), state.tetromino.get_color());
                }
            }
        }
    }
}
