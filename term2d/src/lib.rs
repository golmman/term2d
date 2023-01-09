use std::io::stdin;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use controller::Controller;
use model::config::Config;
use model::event::Event;
use termion::input::TermRead;
use view::canvas::Canvas;
use view::screen::RawTerminalScreen;

pub mod controller;
pub mod model;
pub mod view;

pub fn run<T: Canvas, C: Controller<T>>(controller: C) {
    run_with_config(controller, Config::default());
}

pub fn run_with_config<T: Canvas, C: Controller<T>>(mut controller: C, config: Config) {
    let screen = RawTerminalScreen::new(config.screen_drop_strings);
    controller.get_canvas().init(screen);

    let (sender, receiver) = sync_channel::<Event>(1024);
    let elapse_sender = sender.clone();
    let key_sender = sender.clone();
    let resize_sender = sender.clone();

    thread::spawn(move || send_elapse_events(elapse_sender, config.fps));
    thread::spawn(move || send_key_events(key_sender));
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

fn send_resize_events(sync_sender: SyncSender<Event>) {
    let _ = unsafe {
        signal_hook::low_level::register(signal_hook::consts::SIGWINCH, move || {
            sync_sender.send(Event::Resize).unwrap();
        })
    };
}
