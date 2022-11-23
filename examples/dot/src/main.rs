use term2d::{
    color::{Color, Rgba},
    point::Point,
    view::canvas::{halfblock::HalfblockCanvas, Canvas},
    view::screen::DefaultScreen,
    Controller, Event, Key,
};

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

    fn init(&mut self, screen: DefaultScreen) -> term2d::Config {
        self.canvas.init(screen);
        term2d::Config { fps: 10 }
    }
}

fn main() {
    let controller = DotController {
        frame: 0,
        canvas: HalfblockCanvas::new(),
    };
    term2d::run(controller);
}
