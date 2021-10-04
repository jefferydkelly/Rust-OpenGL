use super::{aabb::AABB, transform::Transform};
use super::ray::Ray;
use glm::{Vec3, Mat4};
use once_cell::sync::OnceCell;

static mut PHSYICS_MANAGER:OnceCell<Physics> = OnceCell::new();

pub struct Physics {
    bodies:Vec<AABB>,
    projection:Mat4,
    view:Mat4,
    pub max_angle:f32,
    collisions:Vec<Option<AABB>>
}

impl Physics {
    pub fn create_instance() {
        let angel:f32 = (30.0 as f32).to_radians();
        let many = Physics {
            bodies:Vec::new(),
            max_angle: angel.cos(),
            projection: Mat4::identity(),
            view: Mat4::identity(),
            collisions: Vec::new()
        };

        unsafe {
            PHSYICS_MANAGER.set(many);
        }
    }

    pub fn get_instance() -> &'static mut Physics {
        unsafe {
            PHSYICS_MANAGER.get_mut().expect("Physics Manager has not been created")
        }
    }

    pub fn set_matrix(&mut self, p:Mat4, v:Mat4) {
        self.projection = p;
        self.view = v;
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


    /*
        Checks for collision among the Axis-Aligned Bounding Boxes and notes them in the collisions vector if they occur 
    */
    pub fn collision_check(&mut self) {
        
        for i in 0..self.collisions.len() {
            self.collisions[i] = None;
        }
        for i in 0..self.bodies.len() {
            let body_a = self.bodies[i];
            for j in (i+1)..self.bodies.len() {
                let body_b = self.bodies[j];

                if body_a.overlaps(body_b) {
                    self.collisions[i] = Some(body_b);
                    self.collisions[j] = Some(body_a);
                }
            }
        }
        
    }

    /*
        Adds a new body to the list of bodies tracked
        trans - The transform for the parent object of the body
        min - The minimum point on the Axis-Aligned Bounding Body
        max - The maximum point of the AABB
        return - The index of the new body 
    */
    pub fn add_body(&mut self, trans:Transform, min:Vec3, max:Vec3)-> usize {

       
        let newb = AABB::new(min, max, trans);
        self.bodies.push(newb);
        self.collisions.push(None);
        self.bodies.len() - 1
    }

    /*
        Gets the body from the list
        index - The index of the body in the vector
        return - The Axis-Aligned Bounding Box 
    */
    pub fn get_body(&self, index:usize) -> AABB {
        self.bodies[index]
    }

    /*
        Updates the information of the body at index using the given transform
        index - The index of the body in the vector
        trans - The transform for the parent object of the body
    */
    pub fn update_body(&mut self, index:usize, trans:Transform) {
        self.bodies[index].update(trans);
    }

    /*
        Checks if a collision has occured with the body and index since the last update
        index - The index of the checked body in the vector
        return - The other body if there is a collision.  None otherwise. 
    */
    pub fn check_for_collision(&self, index:usize) -> Option<AABB> {
        if index < self.collisions.len() {
            return self.collisions[index];
        }

        None
    }
}