use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use std::fs::File;

use term2d::{
    color::Rgba,
    point::Point,
    renderer::{half_block_renderer::HalfBlockRenderer, Renderer},
    run,
    screen::DefaultScreen,
    Controller, Event, Key,
};

struct Image {
    size: Point,
    pixels: Vec<Rgba>,
}

struct DotController {
    frame: u32,
    gif: Vec<Image>,
    renderer: HalfBlockRenderer,
}

impl DotController {
    fn new(gif: Vec<Image>) -> Self {
        Self {
            frame: 0,
            gif,
            renderer: HalfBlockRenderer::new(),
        }
    }
}

impl Controller for DotController {
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

        self.renderer.clear();
        self.renderer.draw_text_transparent(
            Point::new(2, 0),
            Rgba::white(),
            format!(
                "press 'q' to quit, frame: {}, {:?}",
                self.frame, self.gif[0].size
            ),
        );

        let gif_x = 2;
        let gif_y = 3;
        let image = &self.gif[self.frame as usize % self.gif.len()];
        for y in 0..image.size.height() {
            for x in 0..image.size.width() {
                let index = (x + y * image.size.width()) as usize;
                let rgb = image.pixels[index];
                self.renderer
                    .draw_pixel(Point::new(gif_x + x, gif_y + y), rgb);
            }
        }

        self.renderer.display();

        self.frame += 1;

        true
    }

    fn get_config(&self) -> term2d::Config {
        term2d::Config { fps: 10 }
    }

    fn init(&mut self, screen: DefaultScreen) {
        self.renderer.init(screen);
    }
}

fn main() {
    let gif = load_gif();
    let controller = DotController::new(gif);
    run(controller);
}

fn load_gif() -> Vec<Image> {
    let mut gif = Vec::new();

    let file_in = File::open("examples/animation/data/walk.gif").unwrap();
    let decoder = GifDecoder::new(file_in).unwrap();
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().unwrap();

    for frame in frames {
        let img = frame.buffer();
        let mut image = Image {
            size: Point::from(img.dimensions()),
            pixels: Vec::new(),
        };

        for y in 0..img.dimensions().1 {
            for x in 0..img.dimensions().0 {
                let image::Rgba([r, g, b, a]) = img.get_pixel(x, y);

                if *a == 0 {
                    image.pixels.push(Rgba { r: 96, g: 96, b: 96 });
                } else {
                    image.pixels.push(Rgba {
                        r: *r,
                        g: *g,
                        b: *b,
                    });
                }
            }
        }

        gif.push(image);
    }

    gif
}
