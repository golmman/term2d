use term2d::model::point::Point;
use term2d::model::rgba::Rgba;

use self::mesh::Face;
use self::mesh::Mesh;
use self::mesh::Vertex;

pub mod init;
pub mod mesh;

pub struct MyModel {
    pub mesh: Mesh,
    pub pixel_point: Point,
}

impl MyModel {
    pub fn new() -> Self {
        let mesh = Mesh {
            faces: vec![
                Face::new(0, 2, 1, Rgba::red()),
                Face::new(0, 1, 3, Rgba::green()),
                Face::new(0, 3, 2, Rgba::blue()),
                Face::new(1, 2, 3, Rgba::yellow()),
            ],
            vertices: vec![
                Vertex::new(0.0, 0.0, 0.0),
                Vertex::new(1.0, 0.0, 0.0),
                Vertex::new(0.0, 1.0, 0.0),
                Vertex::new(0.0, 0.0, 1.0),
            ],
        };

        Self {
            mesh,
            pixel_point: Point::new(0, 0),
        }
    }
}
