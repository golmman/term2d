use term2d::{
    color::{Color, Rgba},
    point::Point,
    rect::Rect,
    view::canvas::{halfblock::HalfblockCanvas, Canvas},
    view::screen::DefaultScreen,
    Controller, Event, Key,
};

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

impl Controller for AlphaController {
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

    fn get_config(&self) -> term2d::Config {
        term2d::Config { fps: 10 }
    }

    fn init(&mut self, screen: DefaultScreen) {
        self.canvas.init(screen);
    }
}

fn main() {
    let controller = AlphaController::new();
    term2d::run(controller);
}
