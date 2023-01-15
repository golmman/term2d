use term2d::model::rgba::Rgba;

use super::tetromino::TetrominoState;

pub struct State {
    pub frame: u32,
    pub playfield: Vec<Option<Rgba>>,
    pub tetromino: TetrominoState,
}

impl State {
    pub fn new() -> Self {
        Self {
            frame: 0,
            playfield: vec![None; 10 * 20],
            tetromino: TetrominoState::new(),
        }
    }

    pub fn rotate_left(&mut self) {
        self.tetromino.rotation += 1;
        if self.tetromino.rotation >= 4 {
            self.tetromino.rotation = 0;
        }
    }

    pub fn rotate_right(&mut self) {
        self.tetromino.rotation -= 1;
        if self.tetromino.rotation < 0 {
            self.tetromino.rotation = 3;
        }
    }
}
