use super::face::Face;
use super::vertex::Vertex;

pub struct Mesh {
    pub faces: Vec<Face>,
    pub vertices: Vec<Vertex>,
}
