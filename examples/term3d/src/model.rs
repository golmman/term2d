use std::f32::consts::PI;

use term2d::model::point::Point;
use term2d::model::rgba::Rgba;

use self::camera::Camera;
use self::face::Face;
use self::mesh::Mesh;
use self::vertex::Vertex;

pub mod camera;
pub mod face;
pub mod init;
pub mod mesh;
pub mod vertex;

pub struct MyModel {
    pub camera: Camera,
    pub mesh: Mesh,
    pub pixel_point: Point,
}

impl MyModel {
    pub fn new() -> Self {
        let camera = Camera {
            far: 100.0,
            fov: PI / 2.0,
            look_at: Vertex::new(0.0, 0.0, 0.0),
            near: 2.0,
            position: Vertex::new(-4.0, 0.0, 0.5),
        };

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
            camera,
            mesh,
            pixel_point: Point::new(0, 0),
        }
    }
}
