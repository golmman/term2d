use term2d::model::point::Point;
use term2d::App;

use super::MyModel;

pub fn init_model(_app: &App) -> MyModel {
    MyModel {
        pixel_point: Point::new(0, 0),
    }
}
