extern crate nalgebra_glm as glm;
use glm::{Vec3, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    position:Vec3,
    pub normal:Vec3,
    pub tex_coords:Vec2,
}

impl Vertex {
    pub fn new(p:Vec3, n:Vec3, tc:Vec2)-> Self {
        Self {
            position:p,
            normal:n,
            tex_coords:tc,
        }
    }
}