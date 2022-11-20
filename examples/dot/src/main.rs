use term2d::{
    color::{Color, Rgba},
    point::Point,
    view::canvas::{fullblock::FullblockCanvas, Canvas},
    view::screen::DefaultScreen,
    Controller, Event, Key,
};

struct DotController {
    frame: u32,
    canvas: FullblockCanvas,
}

impl DotController {
    fn new() -> Self {
        Self {
            frame: 0,
            canvas: FullblockCanvas::new(),
        }
    }
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

        self.canvas.draw_pixel(&Point::new(0, 0), &Rgba::red());
        self.canvas.draw_pixel(&Point::new(1, 1), &Rgba::red());
        self.canvas.draw_pixel(&Point::new(2, 2), &Rgba::red());
        self.canvas.draw_pixel(&Point::new(3, 3), &Rgba::red());
        self.canvas.draw_pixel(&Point::new(4, 4), &Rgba::red());
        self.canvas.draw_pixel(&Point::new(5, 5), &Rgba::red());
        self.canvas.draw_pixel(&Point::new(6, 6), &Rgba::red());

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
    let controller = DotController::new();
    term2d::run(controller);
}
