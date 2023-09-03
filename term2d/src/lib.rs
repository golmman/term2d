use std::io::stdin;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use model::config::Config;
use model::event::Event;
use termion::input::TermRead;
use view::canvas::halfblock::HalfblockCanvas;
use view::canvas::Canvas;
use view::screen::RawTerminalScreen;

pub mod controller;
pub mod model;
pub mod view;

pub type DefaultCanvas = HalfblockCanvas;

pub type ModelFn<Model> = fn(&App) -> Model;
pub type ViewFn<Model, Canvas = DefaultCanvas> = fn(&App, &Model, &mut Canvas);
pub type EventFn<Model> = fn(&App, &mut Model, Event) -> bool;

pub struct App {
    pub config: Config,
    pub frame_count: u64,

    // make sure App is never constructed, AppBuilder has to be used
    _private_constructor: i32,
}

impl App {
    fn new(config: Config) -> Self {
        Self {
            config,
            frame_count: 0,

            _private_constructor: 0,
        }
    }
}

pub struct AppBuilder<M, C = DefaultCanvas>
where
    C: Canvas,
{
    pub canvas: C,
    pub config: Config,
    pub model_fn: ModelFn<M>,
    pub view_fn: ViewFn<M, C>,
    pub event_fn: EventFn<M>,
}

impl<M> AppBuilder<M> {
    pub fn new(model_fn: ModelFn<M>) -> Self {
        Self {
            canvas: HalfblockCanvas::new(),
            config: Config::default(),
            model_fn,
            view_fn: |_a, _m, _c| {},
            event_fn: |_a, _m, _e| true,
        }
    }

    pub fn run(mut self) {
        let mut app = App::new(self.config.clone());
        let screen = RawTerminalScreen::new(self.config.screen_drop_strings);
        self.canvas.init(screen);

        let (sender, receiver) = sync_channel::<Event>(1024);
        let elapse_sender = sender.clone();
        let key_sender = sender.clone();
        let resize_sender = sender.clone();

        thread::spawn(move || send_elapse_events(elapse_sender, self.config.fps));
        thread::spawn(move || send_key_events(key_sender));
        thread::spawn(move || send_resize_events(resize_sender));

        let mut model = (self.model_fn)(&app);

        (self.event_fn)(&app, &mut model, Event::Resize);

        loop {
            let event = receiver.recv().unwrap();
            if !(self.event_fn)(&app, &mut model, event) {
                return;
            };
            (self.view_fn)(&app, &model, &mut self.canvas);
            app.frame_count += 1;
        }
    }

    pub fn fps(self, fps: u32) -> Self {
        let AppBuilder {
            canvas,
            config: Config {
                screen_drop_strings,
                ..
            },
            model_fn,
            view_fn,
            event_fn,
            ..
        } = self;

        AppBuilder {
            canvas,
            config: Config {
                fps,
                screen_drop_strings,
            },
            model_fn,
            view_fn,
            event_fn,
        }
    }

    pub fn event(self, event_fn: EventFn<M>) -> Self {
        let AppBuilder {
            canvas,
            config,
            model_fn,
            view_fn,
            ..
        } = self;

        AppBuilder {
            canvas,
            config,
            model_fn,
            view_fn,
            event_fn,
        }
    }

    pub fn view(self, view_fn: ViewFn<M>) -> Self {
        let AppBuilder {
            canvas,
            config,
            model_fn,
            event_fn,
            ..
        } = self;

        AppBuilder {
            canvas,
            config,
            model_fn,
            view_fn,
            event_fn,
        }
    }
}

fn send_elapse_events(sender: SyncSender<Event>, fps: u32) {
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
