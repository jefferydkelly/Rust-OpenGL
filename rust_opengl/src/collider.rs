use glm::{Vec2};

#[derive(Clone, Copy)]
pub struct Collider {
    pub position:Vec2,
    pub size:Vec2
}

impl Collider {
    pub fn new(pos:Vec2, scale:Vec2) -> Self {
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