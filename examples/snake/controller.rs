use crate::renderer::SnakeRenderer;
use crate::state::State;
use term2d::{screen::DefaultScreen, Controller, Event, Key};

pub struct SnakeController {
    renderer: SnakeRenderer,
    state: State,
}

impl SnakeController {
    pub fn new() -> Self {
        Self {
            renderer: SnakeRenderer::new(),
            state: State::new(),
        }
    }

    fn resize(&mut self) {
        let screen_size = self.renderer.resize();
        self.state.resize(screen_size);
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
            Event::Resize => self.resize(),
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
