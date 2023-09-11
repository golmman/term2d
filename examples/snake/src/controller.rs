use crate::state::SnakeModel;
use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::App;

pub fn update_model(_app: &App, model: &mut SnakeModel, event: Event) -> bool {
    match event {
        Event::Key(key) => match key {
            Key::Char('q') => return false,
            Key::Ctrl('c') => return false,

            Key::Char('r') => model.reset(),

            Key::Char('w') | Key::Char('k') | Key::Up => model.go_up(),
            Key::Char('s') | Key::Char('j') | Key::Down => model.go_down(),
            Key::Char('a') | Key::Char('h') | Key::Left => model.go_left(),
            Key::Char('d') | Key::Char('l') | Key::Right => model.go_right(),

            _ => {}
        },
        Event::Resize(size) => model.resize(&size),
        Event::Elapse => model.update(),
    }

    true
}
