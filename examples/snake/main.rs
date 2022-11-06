use controller::SnakeController;
use term2d::run;

mod controller;
mod random;
mod renderer;
mod state;

fn main() {
    let controller = SnakeController::new();
    run(controller);
}
