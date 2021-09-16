use glm::Vec3;
use crate::engine::ray::Ray;
pub struct AABB {
    min:Vec3,
    max:Vec3,
    center:Vec3
}

impl AABB {
    pub fn new(mi:Vec3, ma:Vec3) -> Self {
        Self {
            min:mi,
            max:ma,
            center: mi + (ma - mi).scale(0.5)
        }
    }

    pub fn intersects_with_ray(&self, ray:Ray)->bool {
        
        let origin = ray.get_origin();
        let direction = ray.get_direction();
        
        let (mut x_min, mut x_max) = self.get_max_and_min(self.min.x, self.max.x, origin.x, direction.x);
        let (y_min, y_max) = self.get_max_and_min(self.min.y, self.max.y, origin.y, direction.y);

        if (x_min > y_max) || (y_min > x_max) {
            return false;
        }

        if y_min > x_min {
            x_min = y_min;
        }

        if y_max < x_max {
            x_max = y_max;
        }

        let (z_min, z_max) = self.get_max_and_min(self.min.z, self.max.z, origin.z, direction.z);

        if (x_min > z_max) || (z_min > x_max) {
            return false;
        }
        
        true
    }

    fn get_max_and_min(&self, min:f32, max:f32, origin:f32, direction:f32) -> (f32, f32) {
        let t_min = (min - origin) / direction;
        let t_max = (max - origin) / direction;

        if t_min > t_max {
           return (t_max, t_min)
        }


        return (t_min, t_max)
    }

    pub fn get_center(&self)->Vec3 {
        self.center
    }
}