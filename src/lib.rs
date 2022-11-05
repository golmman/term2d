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
pub mod screen;

pub type Key = termion::event::Key;

#[derive(Clone, Debug)]
pub enum Event {
    Key(Key),
    Resize,
    Elapse,
}

pub struct Context {
    pub event: Event,
    pub screen: DefaultScreen,
}

pub struct Config {
    pub fps: u16,
}

pub trait Controller {
    fn update(&mut self, context: &mut Context) -> bool;
    fn get_config(&self) -> Config;
}

pub fn run<T: Controller>(mut controller: T) {
    let screen = DefaultScreen::new();
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

    let mut context = Context {
        event: Event::Resize,
        screen,
    };

    loop {
        context.event = receiver.recv().unwrap();
        if !controller.update(&mut context) {
            break;
        }
    }
}

fn send_elapse_events(sender: SyncSender<Event>, fps: u16) {
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
