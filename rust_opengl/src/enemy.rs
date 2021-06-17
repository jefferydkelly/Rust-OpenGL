use crate::texture::Texture;
use glm::{Vec2,Vec3, vec2, vec3};
use crate::collider::Collider;
use crate::sprite_renderer::SpriteRenderer;

pub struct Enemy {
    sprite:Texture,
    position:Vec2,
    size:Vec2,
    velocity:Vec2,
    color:Vec3,
    rotation:f32,
    pub collider:Collider,
    is_visible:bool
}

impl Enemy {
    pub fn new(pos:Vec2, scale:Vec2, rot:f32, spr:Texture)->Self {
        Self {
            position:pos,
            size:scale,
            velocity: vec2(0.0, 0.0),
            color: vec3(1.0,1.0,1.0),
            rotation:rot,
            sprite:spr,
            collider: Collider::new(pos, scale),
            is_visible:true
        }
    }

    pub fn draw_sprite(&self, renderer:&SpriteRenderer) {
        if self.is_visible {
            renderer.draw_sprite(self.sprite, self.position, self.size, self.rotation, self.color);
        }
    }

    pub fn update(&mut self, dt:f32) {
        if self.is_visible {
            self.position += self.velocity.scale(dt);
            if self.is_offscreen() {
                if self.position.y < 0.0 {
                    self.position.y = 600.0;
                }
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

    pub fn set_velocity(&mut self, vel:Vec2) {
        self.velocity = vel;
    }

    pub fn set_position(&mut self, pos:Vec2) {
        self.position = pos;
    }

    pub fn set_rotation(&mut self, rot:f32) {
        self.rotation = rot;
    }

    fn is_offscreen(&self)->bool {
        return self.position.x < 0.0 || self.position.x > 800.0 || self.position.y < 0.0 || self.position.y > 600.0;
    }
}