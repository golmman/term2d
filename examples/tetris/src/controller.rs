use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::view::canvas::halfblock::HalfblockCanvas;

use crate::model::state::State;
use crate::view::Renderer;

pub struct Controller {
    state: State,
    renderer: Renderer,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            renderer: Renderer::new(),
        }
    }
}

impl term2d::controller::Controller<HalfblockCanvas> for Controller {
    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => match key {
                Key::Char('q') => return false,
                Key::Ctrl('c') => return false,

                Key::Char('h') => self.state.rotate_left(),
                Key::Char('l') => self.state.rotate_right(),

                _ => {}
            },
            Event::Resize => {}
            Event::Elapse => {}
        }

        self.renderer.draw(&self.state);

        self.state.frame += 1;

        true
    }

    fn get_canvas(&mut self) -> &mut HalfblockCanvas {
        &mut self.renderer.canvas
    }
}
