use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use std::fs::File;
use std::path::Path;
use term2d::model::image::Video;

use term2d::{
    color::{Color, Rgba},
    point::Point,
    rect::Rect,
    view::canvas::{halfblock::HalfblockCanvas, Canvas},
    view::screen::DefaultScreen,
    Controller, Event, Key,
};

struct AnimationController {
    video: Video,
    canvas: HalfblockCanvas,
}

impl AnimationController {
    fn new(video: Video) -> Self {
        Self {
            video,
            canvas: HalfblockCanvas::new(),
        }
    }
}

impl Controller for AnimationController {
    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => match key {
                Key::Char('q') => return false,
                Key::Ctrl('c') => return false,
                _ => {}
            },
            Event::Resize => {}
            Event::Elapse => {}
        }

        self.canvas.clear();

        self.canvas.draw_text(
            &Point::new(2, 0),
            &Color {
                fg: Rgba::white(),
                bg: Rgba::transparent(),
            },
            &format!(
                "press 'q' to quit, frame: {}, {:?}",
                self.video.frame,
                self.video.images.len()
            ),
        );

        self.canvas.draw_rect(
            &Rect::new(2, 2, 20, 20),
            &Rgba {
                r: 96,
                g: 96,
                b: 96,
                a: 255,
            },
        );

        self.canvas.draw_video(&Point::new(2, 3), &mut self.video);

        self.canvas.display();

        true
    }

    fn get_config(&self) -> term2d::Config {
        term2d::Config { fps: 10 }
    }

    fn init(&mut self, screen: DefaultScreen) {
        self.canvas.init(screen);
    }
}

fn main() {
    let raw_gif = load_gif_raw("examples/animation/data/walk.gif");
    let video = Video::from(raw_gif);
    let controller = AnimationController::new(video);
    term2d::run(controller);
}

fn load_gif_raw<T: AsRef<Path>>(path: T) -> Vec<(u32, u32, Vec<u8>)> {
    let file_in = File::open(path).unwrap();
    GifDecoder::new(file_in)
        .unwrap()
        .into_frames()
        .map(|frame| {
            let frame = frame.unwrap();
            let buffer = frame.buffer();
            let width = buffer.width();
            let height = buffer.height();
            let raw = frame.into_buffer().into_raw();
            (width, height, raw)
        })
        .collect()
}
