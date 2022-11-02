use term2d::{add, screen::DefaultScreen, TerminalEvent, run_term2d};
use termion::event::Key;

struct Controller {
    screen: Option<DefaultScreen>,
}

impl term2d::Controller for Controller {
    fn handle_event(&mut self, event: TerminalEvent) -> bool {
        match event {
            TerminalEvent::Key(key) => match key {
                Key::Char('q') => return false,
                Key::Ctrl('c') => return false,
                _ => {}
            },
            TerminalEvent::Resize => {}
            TerminalEvent::Elapse => {}
        }

        //self.renderer.display(&self.state);

        true
    }

    fn set_screen(&mut self, screen: DefaultScreen) {
        self.screen = Some(screen);
    }
}

fn main() {
    let controller = Controller { screen: None };
    run_term2d(controller);
}
