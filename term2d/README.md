# term2d

A simple 2d drawing engine for terminal emulators.

Example which draws some text and a red pixel:

```rust
use term2d::controller::Controller;
use term2d::model::color::Color;
use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;

struct DotController {
    frame: u32,
    canvas: HalfblockCanvas,
}

impl Controller<HalfblockCanvas> for DotController {
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
            &format!("press 'q' to quit, frame: {}", self.frame),
        );
        self.canvas.draw_pixel(&Point::new(10, 7), &Rgba::red());
        self.canvas.display();

        self.frame += 1;

        true
    }

    fn get_canvas(&mut self) -> &mut HalfblockCanvas {
        &mut self.canvas
    }
}

fn main() {
    let controller = DotController {
        frame: 0,
        canvas: HalfblockCanvas::new(),
    };
    term2d::run(controller);
}
```
