extern crate gl;

extern crate nalgebra_glm as glm;
use core::f32;

use glm::{vec2, vec3, Vec2, Vec3};

use crate::texture::Texture;
use crate::sprite_renderer::SpriteRenderer;
use crate::collider::Collider;
use crate::transform2d::Transform2D;

#[derive(Clone)]
pub struct GameObject {
    transform:Transform2D,
    pub velocity:Vec3,
    pub color:Vec3,
    sprite:Texture,
    pub collider:Collider,
    is_visible:bool
}

impl GameObject {
    pub fn new(pos:Vec3, scale:Vec3, rot:f32, spr:Texture)->GameObject {
        GameObject {
            transform:Transform2D::new(vec3(pos.x, pos.y, 0.0), rot, scale),
            velocity: vec3(0.0, 0.0, 0.0),
            color: vec3(1.0,1.0,1.0),
            sprite:spr,
            collider: Collider::new(pos, scale),
            is_visible:true
        }
    }

    pub fn draw_sprite(&self, renderer:&SpriteRenderer) {
        if self.is_visible {
            renderer.draw_sprite(self.sprite, self.transform, self.color);
        }
    }

    pub fn update(&mut self, dt:f32) {
        if self.is_visible {
            let mut position:Vec3 = self.transform.get_translation();
            position += self.velocity.scale(dt);
            self.transform.update_translation(position);
            self.collider.position = position;
            if self.is_offscreen() {
                self.is_visible = false;
            }
        }
    }

    pub fn set_visibility(&mut self, visible:bool) {
        self.is_visible = visible;
    }

    pub fn is_visible(&mut self)-> bool {
        self.is_visible
    }

    pub fn set_color(&mut self, color:Vec3) {
        self.color = color;
    }

    pub fn set_velocity(&mut self, vel:Vec3) {
        self.velocity = vel;
    }

    pub fn set_position(&mut self, pos:Vec3) {
        self.transform.update_translation(pos);
    }

    pub fn set_rotation(&mut self, rot:f32) {
        self.transform.update_rotation(rot);
    }

    fn is_offscreen(&self)->bool {
        let position = self.transform.get_translation();
        return position.x < 0.0 || position.x > 800.0 || position.y < 0.0 || position.y > 600.0;
    }
}

