use super::key::Key;
use super::point::Point;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Key(Key),
    Resize(Point),
    Elapse,
}
