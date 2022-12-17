use term2d::controller::Controller;
use term2d::model::color::Color;
use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::model::point::Point;
use term2d::model::rect::Rect;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;

struct AlphaController {
    frame: u32,
    canvas: HalfblockCanvas,
}

impl AlphaController {
    fn new() -> Self {
        Self {
            frame: 0,
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

        self.canvas.draw_rect(
            &Rect::new(3, 3, 15, 10),
            &Rgba {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
        );
        self.canvas.draw_rect(
            &Rect::new(12, 5, 15, 10),
            &Rgba {
                r: 0,
                g: 255,
                b: 0,
                a: 128,
            },
        );
        self.canvas.draw_rect(
            &Rect::new(8, 8, 15, 15),
            &Rgba {
                r: 0,
                g: 0,
                b: 255,
                a: 128,
            },
        );

        self.canvas
            .draw_line(&Point::new(10, 10), &Point::new(30, 17), &Rgba::red());

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
    term2d::run(controller);
}
