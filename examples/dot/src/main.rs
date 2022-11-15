use term2d::{
    color::Rgba,
    point::Point,
    run,
    screen::DefaultScreen,
    view::canvas::{fullblock::FullblockCanvas, Canvas},
    Controller, Event, Key,
};

struct DotController {
    frame: u32,
    renderer: FullblockCanvas,
}

impl DotController {
    fn new() -> Self {
        Self {
            frame: 0,
            renderer: FullblockCanvas::new(),
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

        self.renderer.clear();
        self.renderer.draw_text_transparent(
            Point::new(2, 0),
            Rgba::white(),
            format!("press 'q' to quit, frame: {}", self.frame),
        );
        self.renderer.draw_pixel(Point::new(0, 0), Rgba::red());
        self.renderer.draw_pixel(Point::new(1, 1), Rgba::red());
        self.renderer.draw_pixel(Point::new(2, 2), Rgba::red());
        self.renderer.draw_pixel(Point::new(3, 3), Rgba::red());
        self.renderer.draw_pixel(Point::new(4, 4), Rgba::red());
        self.renderer.draw_pixel(Point::new(5, 5), Rgba::red());
        self.renderer.draw_pixel(Point::new(6, 6), Rgba::red());
        self.renderer.display();

        self.frame += 1;

        true
    }

    fn get_config(&self) -> term2d::Config {
        term2d::Config { fps: 10 }
    }

    fn init(&mut self, screen: DefaultScreen) {
        self.renderer.init(screen);
    }
}

fn main() {
    let controller = DotController::new();
    run(controller);
}
