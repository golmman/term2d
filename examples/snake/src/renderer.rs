use term2d::App;
use term2d::model::color::Color;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;

use crate::state::SnakeModel;

pub fn draw_model(_app: &App, model: &SnakeModel, canvas: &mut HalfblockCanvas) {
    canvas.clear();

    draw_frame(model, canvas);
    draw_snake(model, canvas);
    draw_food(model, canvas);
    draw_info(model, canvas);
    draw_game_over(model, canvas);

    canvas.display();
}

fn draw_info(model: &SnakeModel, canvas: &mut HalfblockCanvas) {
    canvas.draw_text(
        &Point::new(2, 2),
        &Color {
            fg: Rgba::white(),
            bg: Rgba::transparent(),
        },
        &format!("press 'q' to quit, snake length: {}", model.snake.len(),),
    );
}

fn draw_food(model: &SnakeModel, canvas: &mut HalfblockCanvas) {
    canvas.draw_pixel(&model.food, &Rgba::red());
}

fn draw_snake(model: &SnakeModel, canvas: &mut HalfblockCanvas) {
    if model.snake.len() == 0 {
        return;
    }

    canvas.draw_pixel(
        &model.snake[0],
        &Rgba {
            r: 32,
            g: 128,
            b: 32,
            a: 255,
        },
    );
    for i in 1..model.snake.len() {
        canvas.draw_pixel(
            &model.snake[i],
            &Rgba {
                r: 64,
                g: 192,
                b: 64,
                a: 255,
            },
        );
    }
}

fn draw_game_over(model: &SnakeModel, canvas: &mut HalfblockCanvas) {
    if !model.game_over {
        return;
    }

    const LINE_1: &str = "      GAME OVER       ";
    const LINE_2: &str = " press 'r' to restart ";
    const LEN: i32 = LINE_1.len() as i32;

    let x = model.screen_size.width() / 2 - LEN / 2;
    let y = model.screen_size.height() / 2 - 2;
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

    canvas.draw_text(&Point::new(x, y), color, LINE_1);
    canvas.draw_text(&Point::new(x, y + 2), color, LINE_2);
}

fn draw_frame(model: &SnakeModel, canvas: &mut HalfblockCanvas) {
    let w = model.screen_size.width();
    let h = model.screen_size.height();
    let color_text = &Color::text();

    for x in 1..w - 1 {
        canvas.draw_char(&Point::new(x, 0), color_text, '\u{2500}');
        canvas.draw_char(&Point::new(x, 4), color_text, '\u{2500}');
        canvas.draw_char(&Point::new(x, h - 1), color_text, '\u{2500}');
    }

    for y in 3..h / 2 - 1 {
        canvas.draw_char(&Point::new(0, y * 2), color_text, '\u{2502}');
        canvas.draw_char(&Point::new(w - 1, y * 2), color_text, '\u{2502}');
    }

    canvas.draw_char(&Point::new(0, 0), color_text, '\u{250C}');
    canvas.draw_char(&Point::new(w - 1, 0), color_text, '\u{2510}');

    canvas.draw_char(&Point::new(0, 2), color_text, '\u{2502}');
    canvas.draw_char(&Point::new(w - 1, 2), color_text, '\u{2502}');

    canvas.draw_char(&Point::new(0, 4), color_text, '\u{251C}');
    canvas.draw_char(&Point::new(w - 1, 4), color_text, '\u{2524}');

    canvas.draw_char(&Point::new(0, h - 1), color_text, '\u{2514}');
    canvas.draw_char(&Point::new(w - 1, h - 1), color_text, '\u{2518}');
}
