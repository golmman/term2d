use term2d::{
    color::Rgb,
    point::Point,
    renderer::{full_block_renderer::FullBlockRenderer, Renderer},
    run,
    screen::DefaultScreen,
    Controller, Event, Key,
};

struct DotController {
    frame: u32,
    renderer: FullBlockRenderer,
}

impl DotController {
    fn new() -> Self {
        Self {
            frame: 0,
            renderer: FullBlockRenderer::new(),
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
            Point::new(0, 0),
            Rgb::white(),
            format!("press 'q' to quit, frame: {}", self.frame),
        );
        self.renderer.draw_pixel(Point::new(5, 5), Rgb::red());
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
