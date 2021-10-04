use glm::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    origin:Vec3,
    direction:Vec3
}

impl Ray {
    /*
        Creates a new ray from the given origin and direction
        o - A vector3 representing the origin of the ray
        d - A vector3 representing the direction of the ray
     */
    pub fn new(o:Vec3, d:Vec3)->Self {
        Self {
            origin:o,
            direction:d
        }
    }

    /*
        Gets the value of the origin
        returns - The vector3 representing the origin of the ray
    */
    pub fn get_origin(&self) -> Vec3 {
        self.origin
    }

    /*
        Gets the value of the direction
        returns - The vector3 representing the direction of the ray
    */
    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }
}