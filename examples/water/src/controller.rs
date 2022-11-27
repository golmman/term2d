use term2d::{point::Point, view::screen::DefaultScreen, Controller, Event, Key};

use crate::{renderer::Renderer, state::State};

pub struct DotController {
    renderer: Renderer,
    state: State,
}

impl DotController {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
            state: State::new(&Point::new(50, 20)),
        }
    }
}

impl Controller for DotController {
    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => match key {
                Key::Char('q') => return false,
                Key::Ctrl('c') => return false,

                Key::Char(' ') => self.state.toggle_dirt(),

                Key::Char('h') => self.state.move_cursor_left(),
                Key::Char('l') => self.state.move_cursor_right(),
                Key::Char('k') => self.state.move_cursor_up(),
                Key::Char('j') => self.state.move_cursor_down(),

                _ => {}
            },
            Event::Resize => {}
            Event::Elapse => {}
        }

        self.renderer.draw(&self.state);

        self.state.frame += 1;

        true
    }

    fn init(&mut self, screen: DefaultScreen) -> term2d::Config {
        self.renderer.init(screen);
        term2d::Config { fps: 10 }
    }
}
