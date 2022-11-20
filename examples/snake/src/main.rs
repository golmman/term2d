use controller::SnakeController;

mod controller;
mod random;
mod renderer;
mod state;

fn main() {
    let controller = SnakeController::new();
    term2d::run(controller);
}
