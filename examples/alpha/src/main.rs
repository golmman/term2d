use std::{cell::RefCell, rc::Rc};

use term2d::{
    color::Rgba,
    point::Point,
    rect::Rect,
    run,
    screen::DefaultScreen,
    view::{
        canvas::{halfblock::HalfblockCanvas, Canvas},
        renderer::{image::ImageRenderer, primitive::PrimitiveRenderer},
    },
    Controller, Event, Key,
};

struct DotController {
    frame: u32,
    canvas: HalfblockCanvas,
    primitive_renderer: PrimitiveRenderer<HalfblockCanvas>,
    image_renderer: ImageRenderer<HalfblockCanvas>,
}

impl DotController {
    fn new() -> Self {
        Self {
            frame: 0,
            canvas: HalfblockCanvas::new(),
            image_renderer: ImageRenderer::new(),
            primitive_renderer: PrimitiveRenderer::new(),
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

        self.canvas.clear();
        self.canvas.draw_text_transparent(
            Point::new(2, 0),
            Rgba::white(),
            format!("press 'q' to quit, frame: {}", self.frame),
        );

        self.canvas.draw_rect(
            Rect::new(3, 3, 15, 10),
            Rgba {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
        );
        self.canvas.draw_rect(
            Rect::new(12, 5, 15, 10),
            Rgba {
                r: 0,
                g: 255,
                b: 0,
                a: 128,
            },
        );
        self.canvas.draw_rect(
            Rect::new(8, 8, 15, 15),
            Rgba {
                r: 0,
                g: 0,
                b: 255,
                a: 128,
            },
        );

        self.canvas.display();

        self.frame += 1;

        true
    }

    fn get_config(&self) -> term2d::Config {
        term2d::Config { fps: 10 }
    }

    fn init(&mut self, screen: DefaultScreen) {
        //self.canvas.init(screen);

        //let h = HalfblockCanvas::from(screen);
        //let x = Rc::new(RefCell::new(h));
        let x = &Rc::from(screen);

        self.image_renderer.init(x);
        self.primitive_renderer.init(x);
    }
}

fn main() {
    let controller = DotController::new();
    run(controller);
}
