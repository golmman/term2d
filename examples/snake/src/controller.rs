use crate::renderer::SnakeRenderer;
use crate::state::State;
use term2d::controller::Controller;
use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::view::canvas::halfblock::HalfblockCanvas;

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

impl Controller<HalfblockCanvas> for SnakeController {
    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => match key {
                Key::Char('q') => return false,
                Key::Ctrl('c') => return false,

                Key::Char('r') => self.state.reset(),

                Key::Char('w') | Key::Char('k') | Key::Up => self.state.go_up(),
                Key::Char('s') | Key::Char('j') | Key::Down => self.state.go_down(),
                Key::Char('a') | Key::Char('h') | Key::Left => self.state.go_left(),
                Key::Char('d') | Key::Char('l') | Key::Right => self.state.go_right(),

                _ => {}
            },
            Event::Resize => self.resize(),
            Event::Elapse => self.state.update(),
        }

        self.renderer.display(&self.state);

        self.state.frame += 1;

        true
    }

    fn get_canvas(&mut self) -> &mut HalfblockCanvas {
        &mut self.renderer.canvas
    }
}
