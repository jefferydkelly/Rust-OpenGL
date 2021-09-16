use super::aabb::AABB;
use super::ray::Ray;
use glm::Vec3;

pub struct Physics {
    bodies:Vec<AABB>,
    pub max_angle:f32
}

impl Physics {
    pub fn new() -> Self {
        let angel:f32 = (30.0 as f32).to_radians();
        Self {
            bodies:Vec::new(),
            max_angle: angel.cos()
        }
    }

    pub fn raycast(&self, origin:Vec3, dir:Vec3, max_distance:f32)->Option<&AABB> {
        let direction = dir.normalize();

        let mut passes:Vec<(&AABB, f32)> = Vec::new();
        for body in self.bodies.iter() {
            let difference = body.get_center() - origin;
            let dot = difference.normalize().dot(&direction);
            let distance = difference.magnitude();
            
            if dot >= self.max_angle && distance <= max_distance {
                passes.push((body, distance));
            }
        }

        passes.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());

        let ray = Ray::new(origin, direction);
        for pass in passes {
            let body = pass.0;
            if body.intersects_with_ray(ray) {
                return Some(body);
            }
        }
        
        return None
    }

    pub fn add_body(&mut self, body:AABB) {
        self.bodies.push(body);
    }
}