use glm::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    origin:Vec3,
    direction:Vec3
}

impl Ray {
    pub fn new(o:Vec3, d:Vec3)->Self {
        Self {
            origin:o,
            direction:d
        }
    }

    pub fn get_origin(&self) -> Vec3 {
        self.origin
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }
}