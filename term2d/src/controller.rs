use crate::model::config::Config;
use crate::model::event::Event;
use crate::view::screen::DefaultScreen;

pub trait Controller {
    fn update(&mut self, event: Event) -> bool;
    fn init(&mut self, screen: DefaultScreen) -> Config;
}
