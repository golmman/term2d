use super::vertex::Vertex;

pub struct Camera {
    pub far: f32,
    pub fov: f32,
    pub look_at: Vertex,
    pub near: f32,
    pub position: Vertex,
}
