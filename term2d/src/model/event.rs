use super::key::Key;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Key(Key),
    Resize,
    Elapse,
}
