use term2d::model::rgba::Rgba;

pub struct Face {
    pub color: Rgba,
    pub vertex_indices: [usize; 3],
}

impl Face {
    pub fn new(first: usize, second: usize, third: usize, color: Rgba) -> Self {
        Self {
            color,
            vertex_indices: [first, second, third],
        }
    }
}
