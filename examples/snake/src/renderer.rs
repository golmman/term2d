use term2d::model::color::Color;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::view::screen::DefaultScreen;

use crate::state::State;

pub struct SnakeRenderer {
    canvas: HalfblockCanvas,
}

impl SnakeRenderer {
    pub fn new() -> Self {
        Self {
            canvas: HalfblockCanvas::new(),
        }
    }

    pub fn init(&mut self, screen: DefaultScreen) {
        self.canvas.init(screen);
    }

    pub fn resize(&mut self) -> Point {
        self.canvas.resize()
    }

    pub fn display(&mut self, state: &State) {
        self.canvas.clear();

        self.draw_frame(state);
        self.draw_snake(state);
        self.draw_food(state);
        self.draw_info(state);
        self.draw_game_over(state);

        self.canvas.display();
    }

    fn draw_info(&mut self, state: &State) {
        self.canvas.draw_text(
            &Point::new(2, 2),
            &Color {
                fg: Rgba::white(),
                bg: Rgba::transparent(),
            },
            &format!("press 'q' to quit, snake length: {}", state.snake.len(),),
        );
    }

    fn draw_food(&mut self, state: &State) {
        self.canvas.draw_pixel(&state.food, &Rgba::red());
    }

    fn draw_snake(&mut self, state: &State) {
        if state.snake.len() == 0 {
            return;
        }

        self.canvas.draw_pixel(
            &state.snake[0],
            &Rgba {
                r: 32,
                g: 128,
                b: 32,
                a: 255,
            },
        );
        for i in 1..state.snake.len() {
            self.canvas.draw_pixel(
                &state.snake[i],
                &Rgba {
                    r: 64,
                    g: 192,
                    b: 64,
                    a: 255,
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
        let color = &Color {
            fg: Rgba {
                r: 16,
                g: 16,
                b: 16,
                a: 255,
            },
            bg: Rgba {
                r: 192,
                g: 192,
                b: 192,
                a: 255,
            },
        };

        self.canvas.draw_text(&Point::new(x, y), color, LINE_1);
        self.canvas.draw_text(&Point::new(x, y + 2), color, LINE_2);
    }

    fn draw_frame(&mut self, state: &State) {
        let w = state.screen_size.width();
        let h = state.screen_size.height();
        let color_text = &Color::text();

        for x in 1..w - 1 {
            self.canvas
                .draw_char(&Point::new(x, 0), color_text, '\u{2500}');
            self.canvas
                .draw_char(&Point::new(x, 4), color_text, '\u{2500}');
            self.canvas
                .draw_char(&Point::new(x, h - 1), color_text, '\u{2500}');
        }

        for y in 3..h / 2 - 1 {
            self.canvas
                .draw_char(&Point::new(0, y * 2), color_text, '\u{2502}');
            self.canvas
                .draw_char(&Point::new(w - 1, y * 2), color_text, '\u{2502}');
        }

        self.canvas
            .draw_char(&Point::new(0, 0), color_text, '\u{250C}');
        self.canvas
            .draw_char(&Point::new(w - 1, 0), color_text, '\u{2510}');

        self.canvas
            .draw_char(&Point::new(0, 2), color_text, '\u{2502}');
        self.canvas
            .draw_char(&Point::new(w - 1, 2), color_text, '\u{2502}');

        self.canvas
            .draw_char(&Point::new(0, 4), color_text, '\u{251C}');
        self.canvas
            .draw_char(&Point::new(w - 1, 4), color_text, '\u{2524}');

        self.canvas
            .draw_char(&Point::new(0, h - 1), color_text, '\u{2514}');
        self.canvas
            .draw_char(&Point::new(w - 1, h - 1), color_text, '\u{2518}');
    }
}
