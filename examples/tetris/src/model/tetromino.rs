use term2d::model::point::Point;
use term2d::model::rgba::Rgba;

// https://strategywiki.org/wiki/Tetris/Rotation_systems

//#[rustfmt::skip]
pub const L: TetrominoData = TetrominoData {
    center: Point::new(1, 1),
    color: Rgba::cyan(),
    pixels: [
        [[0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0], [0, 1, 0, 0]],
        [[1, 1, 1, 1], [0, 0, 1, 0], [0, 0, 0, 0], [0, 1, 0, 0]],
        [[0, 0, 0, 0], [0, 0, 1, 0], [1, 1, 1, 1], [0, 1, 0, 0]],
        [[0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0], [0, 1, 0, 0]],
    ],
};

pub const TETROMINOS: [TetrominoData; 1] = [L];

pub struct TetrominoData {
    pub center: Point,
    pub color: Rgba,
    pub pixels: [[[u8; 4]; 4]; 4],
}

impl TetrominoData {
    pub fn is_pixel_visible(&self, rotation: i32, x: i32, y: i32) -> bool {
        self.pixels[y as usize][rotation as usize][x as usize] == 1
    }
}

pub struct TetrominoState {
    pub kind: usize,
    pub position: Point,
    pub rotation: i32,
}

impl TetrominoState {
    pub fn new() -> Self {
        Self {
            kind: 0,
            position: Point::new(0, 0),
            rotation: 0,
        }
    }

    pub fn is_pixel_visible(&self, x: i32, y: i32) -> bool {
        TETROMINOS[self.kind].is_pixel_visible(self.rotation, x, y)
    }

    pub fn get_color(&self) -> &Rgba {
        &TETROMINOS[self.kind].color
    }
}
