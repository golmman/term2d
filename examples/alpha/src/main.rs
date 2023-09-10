use term2d::model::circle::Circle;
use term2d::model::color::Color;
use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::model::point::Point;
use term2d::model::polygon::Polygon;
use term2d::model::rect::Rect;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::App;
use term2d::AppBuilder;

struct AlphaModel {
    polygon: Polygon,
}

fn init_model(_app: &App) -> AlphaModel {
    let mut polygon = Polygon::new_star();
    polygon += &Point::new(120, 15);
    AlphaModel { polygon }
}

fn event_fn(app: &App, model: &mut AlphaModel, event: Event) -> bool {
    match event {
        Event::Key(key) => match key {
            Key::Char('q') => return false,
            Key::Ctrl('c') => return false,
            _ => {}
        },
        Event::Resize(_) => {}
        Event::Elapse => {
            model.polygon.rotate(app.frame_count as f32 / -11.0);
        }
    }

    true
}

fn view_fn(app: &App, model: &AlphaModel, canvas: &mut HalfblockCanvas) {
    canvas.clear();

    // 3 rectangles blended with alpha
    canvas.draw_rect_fill(
        &Rect::new(3, 3, 15, 10),
        &Rgba {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        },
    );
    canvas.draw_rect_fill(
        &Rect::new(12, 5, 15, 10),
        &Rgba {
            r: 0,
            g: 255,
            b: 0,
            a: 128,
        },
    );
    canvas.draw_rect_fill(
        &Rect::new(8, 8, 15, 15),
        &Rgba {
            r: 0,
            g: 0,
            b: 255,
            a: 128,
        },
    );

    // text
    canvas.draw_text(
        &Point::new(2, 18),
        &Color {
            fg: Rgba::black(),
            bg: Rgba {
                r: 255,
                g: 255,
                b: 255,
                a: 128,
            },
        },
        &format!("press 'q' to quit, frame: {}", app.frame_count),
    );

    // red line
    canvas.draw_line(&Point::new(10, 10), &Point::new(30, 17), &Rgba::red());

    // filled green circle
    canvas.draw_circle_fill(
        &Circle::new(70, 15, (app.frame_count % 27) as i32),
        &Rgba::green(),
    );

    // blue circle boundary
    canvas.draw_circle(
        &Circle::new(50, 15, (app.frame_count % 20) as i32),
        &Rgba {
            r: 0,
            g: 0,
            b: 255,
            a: 128,
        },
    );

    // red rectangle
    canvas.draw_rect(
        &Rect::new(
            100,
            3,
            17 + (10.0 * (app.frame_count as f32 / 7.0).sin()) as i32,
            23,
        ),
        &Rgba::red(),
    );

    // rotating pixel
    canvas.draw_pixel(
        &Point::new(97, 15).rotate(&Point::new(110, 15), app.frame_count as f32 / 10.0),
        &Rgba::yellow(),
    );

    // rotation cyan star
    canvas.draw_polygon(&model.polygon, &Rgba::cyan());

    canvas.display();
}

fn main() {
    AppBuilder::new(init_model)
        .event(event_fn)
        .view(view_fn)
        .run();
}
