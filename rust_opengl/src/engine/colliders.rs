use glm::{vec3, Vec3, vec4, Mat4};
use crate::engine::ray::Ray;

use super::transform::Transform;
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    m_min:Vec3,
    m_max:Vec3,
    w_min:Vec3,
    w_max:Vec3,
    center:Vec3,
    extents:Vec3,
    offset:Vec3,
    true_center:Vec3,
    is_trigger:bool
}

impl AABB {
    pub fn new(mi:Vec3, ma:Vec3, trans:Transform, trigger:bool) -> AABB {
        let mut ext = ma - mi;
        ext *= 0.5;
        let mut abby = AABB {
            m_min:mi,
            m_max:ma,
            center: Vec3::zeros(),
            extents: ext,
            offset:Vec3::zeros(),
            true_center:Vec3::zeros(),
            w_min: mi,
            w_max: ma,
            is_trigger:trigger

        };

        abby.update(trans);
        abby
    }

    /*
        Checks if the ray being cast intersects with this Axis-Aligned Bounding Box
        ray - The ray to be checked against
        return - Whether or not that ray insterects with this AABB
     */
    pub fn intersects_with_ray(&self, ray:Ray)->bool {
        
        let origin = ray.get_origin();
        let direction = ray.get_direction();
        
        let (mut x_min, mut x_max) = self.get_max_and_min(self.w_min.x, self.w_max.x, origin.x, direction.x);
        let (y_min, y_max) = self.get_max_and_min(self.w_min.y, self.w_max.y, origin.y, direction.y);

        if (x_min > y_max) || (y_min > x_max) {
            return false;
        }

        if y_min > x_min {
            x_min = y_min;
        }

        if y_max < x_max {
            x_max = y_max;
        }

        let (z_min, z_max) = self.get_max_and_min(self.w_min.z, self.w_max.z, origin.z, direction.z);

        if (x_min > z_max) || (z_min > x_max) {
            return false;
        }
        
        true
    }

    /*
        Determines the minimum and maximum value of this Axis-Aligned Bounding Box along a one dimensional axis beginning at origin and moving in direction
        min - The AABB's minimum point along the x, y or z axis
        max - The AABB's maximum point along the x, y or z axis 
        origin - The x/y/z component of the ray's origin position
        direction - The x/y/z component of the ray's direction
        return - The maximum and minimum values along the axis.
     */
    fn get_max_and_min(&self, min:f32, max:f32, origin:f32, direction:f32) -> (f32, f32) {
        let t_min = (min - origin) / direction;
        let t_max = (max - origin) / direction;

        if t_min > t_max {
           return (t_max, t_min)
        }


        return (t_min, t_max)
    }

    /*
        Allows outside access of the true_center property
        return - The True Center of the Axis-Aligned Bounding Box.  The place halfway between the maximum point and the minimum point.
     */
    pub fn get_center(&self)->Vec3 {
        self.true_center
    }

    /*
        Updates the values of the Axis-Aligned Bounding Box according to the transform given
        trans - The transform belonging to the parent object
     */
    pub fn update(&mut self, trans:Transform) {
        self.center = trans.translation;

        let mut r_max = self.m_max;
        r_max.x *= trans.scale.x;
        r_max.y *= trans.scale.y;
        r_max.z *= trans.scale.z;

        self.w_max = self.center + r_max;

        let mut r_min = self.m_min;
        r_min.x *= trans.scale.x;
        r_min.y *= trans.scale.y;
        r_min.z *= trans.scale.z;
        self.w_min = self.center + r_min;

        self.extents = (self.w_max - self.w_min) * 0.5;
        self.true_center = self.w_min + self.extents;
        self.offset = self.true_center - self.center;
    }
 
    /*
        Allows outside access of the extent value
        return - A Vec3 representing the half-width/height/depth of the box
     */
    pub fn get_extents(&self) -> Vec3 {
        self.extents
    }

    /*
        Gets the maximum and minimum points on this Axis-Aligned Bounding Box
        return - A tuple contains the maximum point and the minimum point as Vec3s 
     */
    pub fn get_max_and_min_points(&self)->(Vec3,Vec3) {
        (self.w_max, self.w_min)
    }

    /*
        Checks if this Axis-Aligned Bounding Box overlaps the other one
        other - The AABB to be checked against
        return - whether or not they are overlapping  
     */
    pub fn overlaps(&self, other:AABB) -> bool {
        if self.is_trigger && other.is_trigger {
            return false;
        }
        let (o_max, o_min) = other.get_max_and_min_points();
        let (i_max, i_min) = self.get_max_and_min_points();
        
        return i_max.x > o_min.x && i_min.x < o_max.x && i_max.y > o_min.y && i_min.y < o_max.y && i_max.z > o_min.z && i_min.z < o_max.z;
    }

    pub fn is_trigger_collider(&self)->bool {
        self.is_trigger
    }
}