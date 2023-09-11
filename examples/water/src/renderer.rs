use term2d::model::color::Color;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::App;

use crate::state::State;

pub fn draw_model(_app: &App, model: &State, canvas: &mut HalfblockCanvas) {
    canvas.clear();

    draw_world(model, canvas);
    draw_water(model, canvas);
    draw_debug(model, canvas);
    draw_cursor(model, canvas);

    canvas.display();
}

fn draw_cursor(model: &State, canvas: &mut HalfblockCanvas) {
    canvas.draw_pixel(&model.cursor, &Rgba::red());
}

fn draw_debug(model: &State, canvas: &mut HalfblockCanvas) {
    canvas.draw_text(
        &Point::new(2, 0),
        &Color {
            fg: Rgba::white(),
            bg: Rgba::transparent(),
        },
        &format!("press 'q' to quit, frame: {}", model.frame),
    );
    canvas.draw_pixel(&Point::new(10, 7), &Rgba::red());
}

fn draw_world(model: &State, canvas: &mut HalfblockCanvas) {
    canvas.draw_image(
        &Point::new(model.world.pos.x, model.world.pos.y),
        &model.world.image,
    );
}

fn draw_water(model: &State, canvas: &mut HalfblockCanvas) {
    for droplet in &model.world.water {
        let a = Point::new(
            model.world.pos.x + droplet.pos.x,
            model.world.pos.y + droplet.pos.y,
        );
        canvas.draw_pixel(&a, &droplet.rgba);
    }
}
