use model::MyModel;
use model::init::init_model;
use term2d::App;
use term2d::AppBuilder;

use term2d::model::color::Color;

use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;

use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;

pub mod model;

fn update_model(app: &App, model: &mut MyModel, event: Event) -> bool {
    match event {
        Event::Key(key) => match key {
            Key::Char('q') => return false,
            Key::Ctrl('c') => return false,
            _ => {}
        },
        Event::Resize(_) => {}
        Event::Elapse => {
            model.pixel_point.x = 12 + (10.0 * (app.frame_count as f32 / 10.0).cos()) as i32;
            model.pixel_point.y = 12 + (10.0 * (app.frame_count as f32 / 10.0).sin()) as i32;
        }
    }

    true
}

fn draw_model(app: &App, model: &MyModel, canvas: &mut HalfblockCanvas) {
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
        .event(update_model)
        .view(draw_model)
        .fps(20)
        .run();
}
