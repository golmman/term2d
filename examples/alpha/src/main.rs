use term2d::controller::Controller;
use term2d::model::ansiesc::CLEAR_ALL;
use term2d::model::ansiesc::COLOR_RESET;
use term2d::model::ansiesc::CURSOR_GOTO_1_1;
use term2d::model::ansiesc::CURSOR_SHOW;
use term2d::model::circle::Circle;
use term2d::model::color::Color;
use term2d::model::config::Config;
use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::model::point::Point;
use term2d::model::polygon::Polygon;
use term2d::model::rect::Rect;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;

struct AlphaController {
    frame: u32,
    polygon: Polygon,
    canvas: HalfblockCanvas,
}

impl AlphaController {
    fn new() -> Self {
        let mut polygon = Polygon::new_star();
        polygon += &Point::new(120, 15);
        Self {
            frame: 0,
            polygon,
            canvas: HalfblockCanvas::new(),
        }
    }
}

impl Controller<HalfblockCanvas> for AlphaController {
    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => match key {
                Key::Char('q') => return false,
                Key::Ctrl('c') => return false,
                _ => {}
            },
            Event::Resize => {}
            Event::Elapse => {}
        }

        self.canvas.clear();

        // 3 rectangles blended with alpha
        self.canvas.draw_rect_fill(
            &Rect::new(3, 3, 15, 10),
            &Rgba {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
        );
        self.canvas.draw_rect_fill(
            &Rect::new(12, 5, 15, 10),
            &Rgba {
                r: 0,
                g: 255,
                b: 0,
                a: 128,
            },
        );
        self.canvas.draw_rect_fill(
            &Rect::new(8, 8, 15, 15),
            &Rgba {
                r: 0,
                g: 0,
                b: 255,
                a: 128,
            },
        );

        // text
        self.canvas.draw_text(
            &Point::new(2, 18),
            &Color {
                fg: Rgba::black(),
                bg: Rgba {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 128,
                },
            },
            &format!("press 'q' to quit, frame: {}", self.frame),
        );

        // red line
        self.canvas
            .draw_line(&Point::new(10, 10), &Point::new(30, 17), &Rgba::red());

        // filled green circle
        self.canvas.draw_circle_fill(
            &Circle::new(70, 15, (self.frame % 27) as i32),
            &Rgba::green(),
        );

        // blue circle boundary
        self.canvas.draw_circle(
            &Circle::new(50, 15, (self.frame % 20) as i32),
            &Rgba {
                r: 0,
                g: 0,
                b: 255,
                a: 128,
            },
        );

        // red rectangle
        self.canvas.draw_rect(
            &Rect::new(
                100,
                3,
                17 + (10.0 * (self.frame as f32 / 7.0).sin()) as i32,
                23,
            ),
            &Rgba::red(),
        );

        // rotating pixel
        self.canvas.draw_pixel(
            &Point::new(97, 15).rotate(&Point::new(110, 15), self.frame as f32 / 10.0),
            &Rgba::yellow(),
        );

        // rotation cyan star
        self.canvas.draw_polygon(
            &self.polygon.rotate(self.frame as f32 / -11.0),
            &Rgba::cyan(),
        );

        self.canvas.display();

        self.frame += 1;

        true
    }

    fn get_canvas(&mut self) -> &mut HalfblockCanvas {
        &mut self.canvas
    }
}

fn main() {
    let controller = AlphaController::new();
    term2d::run_with_config(
        controller,
        Config {
            fps: 10,
            screen_drop_strings: vec![
                COLOR_RESET.to_string(),
                CLEAR_ALL.to_string(),
                CURSOR_GOTO_1_1.to_string(),
                CURSOR_SHOW.to_string(),
            ],
        },
    );
}
