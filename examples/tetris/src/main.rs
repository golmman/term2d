use controller::Controller;

pub mod controller;
pub mod model;
pub mod view;

fn main() {
    term2d::run(Controller::new());
}
