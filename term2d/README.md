# term2d

A simple 2d drawing engine for terminal emulators.

Example which draws some text and a red pixel:

```rust
use term2d::model::color::Color;
use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::App;
use term2d::AppBuilder;

fn event_fn(_app: &App, _model: &mut (), event: Event) -> bool {
    match event {
        Event::Key(key) => match key {
            Key::Char('q') => return false,
            Key::Ctrl('c') => return false,
            _ => {}
        },
        Event::Resize(_) => {}
        Event::Elapse => {}
    }

    true
}

fn view_fn(app: &App, _model: &(), canvas: &mut HalfblockCanvas) {
    canvas.clear();
    canvas.draw_text(
        &Point::new(2, 0),
        &Color::text(),
        &format!("press 'q' to quit, frame: {}", app.frame_count),
    );
    canvas.draw_pixel(&Point::new(10, 7), &Rgba::red());
    canvas.display();
}

fn main() {
    AppBuilder::new(|_| ())
        .event(event_fn)
        .view(view_fn)
        .fps(20)
        .run();
}
```
