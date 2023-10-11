use term2d::App;

use super::MyModel;

pub fn init_model(_app: &App) -> MyModel {
    MyModel::new()
}
