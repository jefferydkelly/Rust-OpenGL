use glm::{Vec3};

#[derive(Clone, Copy)]
pub struct Collider {
    pub position:Vec3,
    pub size:Vec3
}

impl Collider {
    pub fn new(pos:Vec3, scale:Vec3) -> Self {
        Self {
            position:pos,
            size:scale
        }
    }

    pub fn check_collision(&self, other:Collider) -> bool {
        let total_size = self.size + other.size;
        let distance = self.position - other.position;
        
        if distance.x.abs() <= total_size.x / 2.0 && distance.y.abs() <= total_size.y / 2.0 {
            return true;
        }
        false
    }
}