use term2d::{
    color::Rgba,
    point::Point,
    renderer::{half_block_renderer::HalfBlockRenderer, Renderer},
    run,
    screen::DefaultScreen,
    Controller, Event, Key, rect::Rect,
};

struct DotController {
    frame: u32,
    renderer: HalfBlockRenderer,
}

impl DotController {
    fn new() -> Self {
        Self {
            frame: 0,
            renderer: HalfBlockRenderer::new(),
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

        self.renderer.draw_rect(Rect::new(3, 3, 15, 10), Rgba::red());
        self.renderer.draw_rect(Rect::new(12, 5, 15, 10), Rgba::green());
        self.renderer.draw_rect(Rect::new(8, 8, 10, 15), Rgba::blue());

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
