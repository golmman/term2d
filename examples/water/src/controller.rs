use term2d::controller::Controller;
use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::view::canvas::halfblock::HalfblockCanvas;

use crate::renderer::Renderer;
use crate::state::State;

pub struct DotController {
    renderer: Renderer,
    state: State,
}

impl DotController {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
            state: State::new(),
        }
    }
}

impl Controller<HalfblockCanvas> for DotController {
    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => match key {
                Key::Char('q') => return false,
                Key::Ctrl('c') => return false,

                Key::Char(' ') => self.state.toggle_dirt(),
                Key::Char('w') => self.state.add_droplet(),

                Key::Char('h') => self.state.move_cursor_left(),
                Key::Char('l') => self.state.move_cursor_right(),
                Key::Char('k') => self.state.move_cursor_up(),
                Key::Char('j') => self.state.move_cursor_down(),

                _ => {}
            },
            Event::Resize => {}
            Event::Elapse => {
                self.state.world.simulate_water();
            }
        }

        self.renderer.draw(&self.state);

        self.state.frame += 1;

        true
    }

    fn get_canvas(&mut self) -> &mut HalfblockCanvas {
        &mut self.renderer.canvas
    }
}
