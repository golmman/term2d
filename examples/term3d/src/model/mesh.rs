use term2d::model::rgba::Rgba;

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

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

pub struct Mesh {
    pub faces: Vec<Face>,
    pub vertices: Vec<Vertex>,
}
