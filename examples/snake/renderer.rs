use term2d::{
    color::{Rgb, Color},
    point::Point,
    renderer::{half_block_renderer::HalfBlockRenderer, Renderer},
    screen::DefaultScreen,
};

use crate::state::State;

pub struct SnakeRenderer {
    renderer: HalfBlockRenderer,
}

impl SnakeRenderer {
    pub fn new() -> Self {
        Self {
            renderer: HalfBlockRenderer::new(),
        }
    }

    pub fn init(&mut self, screen: DefaultScreen) {
        self.renderer.init(screen);
    }

    pub fn resize(&mut self) -> Point {
        self.renderer.resize()
    }

    pub fn display(&mut self, state: &State) {
        self.renderer.clear();

        self.draw_frame(state);

        self.renderer.draw_text_transparent(
            Point::new(2, 0),
            Rgb::white(),
            format!("press 'q' to quit, screen_size: ({}, {}), frame: {}", state.screen_size.width(), state.screen_size.height(), state.frame),
        );
        self.renderer.draw_pixel(Point::new(0, 0), Rgb::red());
        self.renderer.draw_pixel(Point::new(1, 1), Rgb::red());
        self.renderer.draw_pixel(Point::new(2, 2), Rgb::red());
        self.renderer.draw_pixel(Point::new(3, 3), Rgb::red());
        self.renderer.draw_pixel(Point::new(4, 4), Rgb::red());
        self.renderer.draw_pixel(Point::new(5, 5), Rgb::red());
        self.renderer.draw_pixel(Point::new(6, 6), Rgb::red());
        self.renderer.display();
    }

    fn draw_frame(&mut self, state: &State) {
        self.renderer.draw_text(Point::new(1, state.screen_size.height() - 1), Color::text(), "------------".to_string());
    }
}
