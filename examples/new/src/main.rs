use std::io::stdin;
use std::process::exit;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use termion::input::TermRead;

use term2d::controller::Controller;
use term2d::model::color::Color;
use term2d::model::config::Config;
use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::fullblock::FullblockCanvas;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::view::screen::RawTerminalScreen;

pub type ModelFn<M> = fn(&App) -> M;
pub type ViewFn<M> = fn(&App, &M, &mut HalfblockCanvas);
pub type EventFn<M> = fn(&App, &mut M, Event) -> bool;
pub type ExitFn<M> = fn(&App, M);

pub type DefaultView = HalfblockCanvas;

struct App {
    config: Config,
    frame_count: u64,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            frame_count: 0,
        }
    }
}

struct AppBuilder<M = ()> {
    canvas: HalfblockCanvas,
    config: Config,
    model_fn: ModelFn<M>,
    view_fn: ViewFn<M>,
    event_fn: EventFn<M>,
}

impl<M> AppBuilder<M> {
    pub fn new(model_fn: ModelFn<M>) -> Self {
        Self {
            canvas: HalfblockCanvas::new(),
            config: Config::default(),
            model_fn,
            view_fn: |a, m, c| {},
            event_fn: |a, m, e| true,
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

struct MyModel {
    pub pixel_point: Point,
}

fn init_model(app: &App) -> MyModel {
    MyModel {
        pixel_point: Point::new(0, 0),
    }
}

fn event_fn(app: &App, model: &mut MyModel, event: Event) -> bool {
    match event {
        Event::Key(key) => match key {
            Key::Char('q') => return false,
            Key::Ctrl('c') => return false,
            _ => {}
        },
        Event::Resize => {}
        Event::Elapse => {
            model.pixel_point.x = 12 + (10.0 * (app.frame_count as f32 / 10.0).cos()) as i32;
            model.pixel_point.y = 12 + (10.0 * (app.frame_count as f32 / 10.0).sin()) as i32;
        }
    }

    true
}

fn view_fn(app: &App, model: &MyModel, canvas: &mut HalfblockCanvas) {
    canvas.clear();
    canvas.draw_pixel(&model.pixel_point, &Rgba::red());
    canvas.draw_text(
        &Point::new(0, 0),
        &Color::text(),
        &format!("press 'q' to quit, frame: {}", app.frame_count),
    );
    canvas.display();
}

fn main() {
    AppBuilder::new(init_model)
        .event(event_fn)
        .view(view_fn)
        .fps(20)
        .run();
}
