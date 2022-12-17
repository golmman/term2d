use crate::model::event::Event;
use crate::view::canvas::Canvas;

pub trait Controller<T: Canvas> {
    fn update(&mut self, event: Event) -> bool;
    fn get_canvas(&mut self) -> &mut T;
}
