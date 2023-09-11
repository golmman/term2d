use controller::update_model;
use renderer::draw_model;
use state::init_model;
use term2d::AppBuilder;

pub mod controller;
pub mod random;
pub mod renderer;
pub mod state;
pub mod water_rules;
pub mod world;

fn main() {
    AppBuilder::new(init_model)
        .event(update_model)
        .view(draw_model)
        .fps(10)
        .run();
}
