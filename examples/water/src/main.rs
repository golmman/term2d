use controller::DotController;

pub mod controller;
pub mod random;
pub mod renderer;
pub mod state;
pub mod water_rules;
pub mod world;

fn main() {
    let controller = DotController::new();
    term2d::run(controller);
}
