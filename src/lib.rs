use std::{sync::mpsc::{sync_channel, SyncSender}, time::Duration, io::stdin, thread::{sleep, self}};

use screen::DefaultScreen;
use termion::event::Key;
use termion::input::TermRead;

pub mod color;
pub mod point;
pub mod screen;

pub enum TerminalEvent {
    Key(Key),
    Resize,
    Elapse,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait Controller {
    fn handle_event(&mut self, event: TerminalEvent) -> bool;
    fn set_screen(&mut self, screen: DefaultScreen);
}

pub fn run_term2d<T: Controller>(mut controller: T) {
    controller.set_screen(DefaultScreen::new());

    let (sender, receiver) = sync_channel::<TerminalEvent>(1024);

    let elapse_sender = sender.clone();
    let key_sender = sender.clone();
    let interrupt_sender = sender.clone();
    let resize_sender = sender.clone();

    thread::spawn(move || send_elapse_events(elapse_sender, 10));
    thread::spawn(move || send_key_events(key_sender));
    thread::spawn(move || send_interrupt_events(interrupt_sender));
    thread::spawn(move || send_resize_events(resize_sender));

    loop {
        let event = receiver.recv().unwrap();
        if !controller.handle_event(event) {
            break;
        }
    }
}

fn send_elapse_events(sender: SyncSender<TerminalEvent>, fps: u16) {
    loop {
        sleep(Duration::from_millis(1000 / fps as u64));
        let _ = sender.send(TerminalEvent::Elapse);
    }
}

fn send_key_events(sender: SyncSender<TerminalEvent>) {
    let stdin = stdin();

    for key in stdin.keys().flatten() {
        let _ = sender.send(TerminalEvent::Key(key));
    }
}

fn send_interrupt_events(sync_sender: SyncSender<TerminalEvent>) {
    // this only exists as a fail safe, terminals in raw mode have to
    // interpret ctrl+c during normal key event handling, so this does
    // nothing in raw mode
    let _ = unsafe {
        signal_hook::low_level::register(signal_hook::consts::SIGINT, move || {
            sync_sender
                .send(TerminalEvent::Key(Key::Char('q')))
                .unwrap();
        })
    };
}

fn send_resize_events(sync_sender: SyncSender<TerminalEvent>) {
    let _ = unsafe {
        signal_hook::low_level::register(signal_hook::consts::SIGWINCH, move || {
            sync_sender.send(TerminalEvent::Resize).unwrap();
        })
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
