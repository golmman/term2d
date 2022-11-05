mod renderer;
mod state;

use renderer::SnakeRenderer;
use state::State;
use term2d::{color::Rgb, point::Point, run, screen::DefaultScreen, Controller, Event, Key};

struct SnakeController {
    renderer: SnakeRenderer,
    state: State,
}

impl SnakeController {
    fn new() -> Self {
        Self {
            renderer: SnakeRenderer::new(),
            state: State { frame: 0 },
        }
    }
}

impl Controller for SnakeController {
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

        self.renderer.display(&self.state);

        self.state.frame += 1;

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
    let controller = SnakeController::new();
    run(controller);
}
