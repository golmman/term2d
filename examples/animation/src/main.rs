use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use image::GenericImageView;
use std::fs::File;
use std::path::Path;
use term2d::model::config::Config;
use term2d::model::image::Image;
use term2d::model::video::Video;

use term2d::controller::Controller;
use term2d::model::color::Color;
use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::model::point::Point;
use term2d::model::rect::Rect;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::view::screen::DefaultScreen;

struct AnimationController {
    cat_video: Video,
    cat_video2: Video,
    deer_image: Image,
    walk_video: Video,
    canvas: HalfblockCanvas,
}

impl AnimationController {
    fn new(cat_video: Video, deer_image: Image, walk_video: Video) -> Self {
        let mut cat_video2 = cat_video.mirror_y();
        cat_video2.frame = 3;

        Self {
            cat_video,
            cat_video2,
            deer_image,
            walk_video,
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
                self.walk_video.frame,
                self.walk_video.images.len()
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

        self.canvas
            .draw_video(&Point::new(10, 6), &mut self.cat_video);
        self.canvas
            .draw_video(&Point::new(19, 6), &mut self.cat_video2);
        self.canvas
            .draw_video(&Point::new(2, 3), &mut self.walk_video);
        self.canvas
            .draw_image(&Point::new(30, 0), &mut self.deer_image);

        self.canvas.display();

        true
    }

    fn init(&mut self, screen: DefaultScreen) -> Config {
        self.canvas.init(screen);
        Config { fps: 10 }
    }
}

fn main() {
    let cat_raw = load_gif_raw("examples/animation/data/cat.gif");
    let walk_raw = load_gif_raw("examples/animation/data/walk.gif");
    let deer_raw = load_image_raw("examples/animation/data/deer.png");

    let cat_video = Video::from(cat_raw);
    let walk_video = Video::from(walk_raw);
    let deer_image = Image::from(deer_raw);

    let controller = AnimationController::new(cat_video, deer_image, walk_video);

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

fn load_image_raw<T: AsRef<Path>>(path: T) -> (u32, u32, Vec<u8>) {
    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();
    let raw = img.into_bytes();
    (width, height, raw)
}
