use std::{
    io::stdin,
    sync::mpsc::{sync_channel, SyncSender},
    thread::{self, sleep},
    time::Duration,
};

use screen::DefaultScreen;
use termion::input::TermRead;

pub mod color;
pub mod point;
pub mod rect;
pub mod screen;
pub mod view;

pub type Key = termion::event::Key;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Key(Key),
    Resize,
    Elapse,
}

pub struct Config {
    pub fps: u16,
}

pub trait Controller {
    fn update(&mut self, event: Event) -> bool;
    fn get_config(&self) -> Config;
    fn init(&mut self, screen: DefaultScreen);
}

pub fn run<C: Controller>(mut controller: C) {
    let screen = DefaultScreen::new();
    controller.init(screen);

    let config = controller.get_config();

    let (sender, receiver) = sync_channel::<Event>(1024);

    let elapse_sender = sender.clone();
    let key_sender = sender.clone();
    let interrupt_sender = sender.clone();
    let resize_sender = sender.clone();

    thread::spawn(move || send_elapse_events(elapse_sender, config.fps));
    thread::spawn(move || send_key_events(key_sender));
    thread::spawn(move || send_interrupt_events(interrupt_sender));
    thread::spawn(move || send_resize_events(resize_sender));

    controller.update(Event::Resize);

    loop {
        let event = receiver.recv().unwrap();
        if !controller.update(event) {
            break;
        }
    }
}

fn send_elapse_events(sender: SyncSender<Event>, fps: u16) {
    if fps <= 0 {
        return;
    }

    loop {
        sleep(Duration::from_millis(1000 / fps as u64));
        let _ = sender.send(Event::Elapse);
    }
}

fn send_key_events(sender: SyncSender<Event>) {
    let stdin = stdin();

    for key in stdin.keys().flatten() {
        let _ = sender.send(Event::Key(key));
    }
}

fn send_interrupt_events(sync_sender: SyncSender<Event>) {
    // this only exists as a fail safe, terminals in raw mode have to
    // interpret ctrl+c during normal key event handling, so this does
    // nothing in raw mode
    let _ = unsafe {
        signal_hook::low_level::register(signal_hook::consts::SIGINT, move || {
            sync_sender.send(Event::Key(Key::Char('q'))).unwrap();
        })
    };
}

fn send_resize_events(sync_sender: SyncSender<Event>) {
    let _ = unsafe {
        signal_hook::low_level::register(signal_hook::consts::SIGWINCH, move || {
            sync_sender.send(Event::Resize).unwrap();
        })
    };
}
