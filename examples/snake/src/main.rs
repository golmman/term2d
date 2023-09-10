use controller::update_model;
use renderer::draw_model;
use state::init_model;
use term2d::AppBuilder;

mod controller;
mod random;
mod renderer;
mod state;

fn main() {
    AppBuilder::new(init_model)
        .event(update_model)
        .view(draw_model)
        .fps(10)
        .run();
}
