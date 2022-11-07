use term2d::{
    color::{Color, Rgb},
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
        self.draw_snake(state);
        self.draw_food(state);
        self.draw_game_over(state);

        self.renderer.draw_text_transparent(
            Point::new(2, 2),
            Rgb::white(),
            format!(
                "press 'q' to quit, screen_size: ({}, {}), frame: {}",
                state.screen_size.width(),
                state.screen_size.height(),
                state.frame
            ),
        );

        //self.renderer.draw_pixel(Point::new(0, 0), Rgb::red());
        //self.renderer.draw_pixel(Point::new(1, 1), Rgb::red());
        //self.renderer.draw_pixel(Point::new(2, 2), Rgb::red());
        //self.renderer.draw_pixel(Point::new(3, 3), Rgb::red());
        //self.renderer.draw_pixel(Point::new(4, 4), Rgb::red());
        //self.renderer.draw_pixel(Point::new(5, 5), Rgb::red());
        //self.renderer.draw_pixel(Point::new(6, 6), Rgb::red());

        self.renderer.display();
    }

    fn draw_food(&mut self, state: &State) {
        self.renderer.draw_pixel(state.food.clone(), Rgb::red());
    }

    fn draw_snake(&mut self, state: &State) {
        if state.snake.len() == 0 {
            return;
        }

        self.renderer.draw_pixel(
            state.snake[state.snake.len() - 1].clone(),
            Rgb {
                r: 32,
                g: 128,
                b: 32,
            },
        );
        for i in 0..state.snake.len() - 1 {
            self.renderer.draw_pixel(
                state.snake[i].clone(),
                Rgb {
                    r: 64,
                    g: 192,
                    b: 64,
                },
            );
        }
    }

    fn draw_game_over(&mut self, state: &State) {
        if !state.game_over {
            return;
        }

        const LINE_1: &str = "      GAME OVER       ";
        const LINE_2: &str = " press 'r' to restart ";
        const LEN: i32 = LINE_1.len() as i32;

        let x = state.screen_size.width() / 2 - LEN / 2;
        let y = state.screen_size.height() / 2 - 2;
        let color = Color {
            fg: Rgb {
                r: 16,
                g: 16,
                b: 16,
            },
            bg: Rgb {
                r: 192,
                g: 192,
                b: 192,
            },
        };

        self.renderer
            .draw_text(Point::new(x, y), color, LINE_1.to_string());
        self.renderer
            .draw_text(Point::new(x, y + 2), color, LINE_2.to_string());
    }

    fn draw_frame(&mut self, state: &State) {
        let w = state.screen_size.width();
        let h = state.screen_size.height();

        for x in 1..w - 1 {
            self.renderer
                .draw_char(Point::new(x, 0), Color::text(), '\u{2500}');
            self.renderer
                .draw_char(Point::new(x, 4), Color::text(), '\u{2500}');
            self.renderer
                .draw_char(Point::new(x, h - 1), Color::text(), '\u{2500}');
        }

        for y in 3..h / 2 - 1 {
            self.renderer
                .draw_char(Point::new(0, y * 2), Color::text(), '\u{2502}');
            self.renderer
                .draw_char(Point::new(w - 1, y * 2), Color::text(), '\u{2502}');
        }

        self.renderer
            .draw_char(Point::new(0, 0), Color::text(), '\u{250C}');
        self.renderer
            .draw_char(Point::new(w - 1, 0), Color::text(), '\u{2510}');

        self.renderer
            .draw_char(Point::new(0, 2), Color::text(), '\u{2502}');
        self.renderer
            .draw_char(Point::new(w - 1, 2), Color::text(), '\u{2502}');

        self.renderer
            .draw_char(Point::new(0, 4), Color::text(), '\u{251C}');
        self.renderer
            .draw_char(Point::new(w - 1, 4), Color::text(), '\u{2524}');

        self.renderer
            .draw_char(Point::new(0, h - 1), Color::text(), '\u{2514}');
        self.renderer
            .draw_char(Point::new(w - 1, h - 1), Color::text(), '\u{2518}');
    }
}
