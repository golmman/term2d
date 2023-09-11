use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::App;

use crate::state::State;

pub fn update_model(_app: &App, model: &mut State, event: Event) -> bool {
    match event {
        Event::Key(key) => match key {
            Key::Char('q') => return false,
            Key::Ctrl('c') => return false,

            Key::Char(' ') => model.toggle_dirt(),
            Key::Char('w') => model.add_droplet(),

            Key::Char('h') => model.move_cursor_left(),
            Key::Char('l') => model.move_cursor_right(),
            Key::Char('k') => model.move_cursor_up(),
            Key::Char('j') => model.move_cursor_down(),

            _ => {}
        },
        Event::Resize(_) => {}
        Event::Elapse => {
            model.world.simulate_water();
        }
    }

    true
}
