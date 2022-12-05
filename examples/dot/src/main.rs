use term2d::controller::Controller;
use term2d::model::color::Color;
use term2d::model::config::Config;
use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::view::screen::DefaultScreen;

struct DotController {
    frame: u32,
    canvas: HalfblockCanvas,
}

impl Controller for DotController {
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
        self.canvas.draw_text(
            &Point::new(2, 0),
            &Color {
                fg: Rgba::white(),
                bg: Rgba::transparent(),
            },
            &format!("press 'q' to quit, frame: {}", self.frame),
        );
        self.canvas.draw_pixel(&Point::new(10, 7), &Rgba::red());
        self.canvas.display();

        self.frame += 1;

        true
    }

    fn init(&mut self, screen: DefaultScreen) -> Config {
        self.canvas.init(screen);
        Config { fps: 10 }
    }
}

fn main() {
    let controller = DotController {
        frame: 0,
        canvas: HalfblockCanvas::new(),
    };
    term2d::run(controller);
}
