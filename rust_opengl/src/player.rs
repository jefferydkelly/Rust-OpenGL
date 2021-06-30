use crate::enemy::Enemy;
use crate::glm::{Vec2, Vec3, vec3, vec2};
use crate::transform2d::{self, Transform2D};
use crate::texture::Texture;
use crate::timer::Timer;
use crate::sprite_renderer::SpriteRenderer;
use crate::traits::{Updated, Rendered};
use crate::input_manager::InputManager;
use crate::collider::Collider;
use crate::game_object::GameObject;

use glfw::Key;
#[derive(Clone, Copy)]
pub struct Player {
    transform:Transform2D,
    velocity:Vec3,
    sprite: Texture,
    timer:Timer,
    pub collider:Collider,
}

impl Player {

    pub fn new(pos:Vec3, scale:Vec3, spr:Texture, bullet_spr:Texture) -> Self {
        let t = Transform2D::new(pos, 0.0, scale);
        Self {
            transform:t,
            sprite:spr,
            velocity:vec3(0.0, 0.0, 0.0),
            timer: Timer::new(),
            collider: Collider::new(pos, scale)
        }
    }

    pub fn init(&mut self) {
        self.timer.start_oneshot(0.25);
    }

    /* 
    pub fn get_position(&self)->Vec2 {
        self.position
    }

    pub fn get_size(&self)-> Vec2 {
        self.size
    }
    */

    pub fn set_position(&mut self, pos:Vec3) {
        self.transform.update_translation(pos);
        self.collider.position = pos;
    }
    /*
    pub fn fire(&mut self) {
        let position:Vec3 = self.transform.get_translation();
        if self.timer.is_ready() {
            for i in 0 .. self.bullets.len() {
                if !self.bullets[i].is_visible() {
                    self.bullets[i].set_position(position + vec3(24.0, -50.0, 0.0));
                    self.bullets[i].set_visibility(true);
                    break;
                }
            }
            self.timer.reset();
        }
    }*/

    pub fn check_collisions(&mut self, enemy:&mut Enemy) {
        if enemy.is_visible() {
            if self.collider.check_collision(enemy.collider) {
                enemy.set_visibility(false);
                return;
            }
        }
    }
}

impl Updated for Player {
    fn update(&mut self, dt:f32) {
       
        if InputManager::instance().get_key_state(glfw::Key::A) {
            self.transform.rotate(1.0);
        }
        let mut position = self.transform.get_translation();
        position += self.velocity.scale(dt);
        self.collider.position = position;
        self.timer.update(dt);
    }
}

impl Rendered for Player {
    fn render(&self, renderer:&SpriteRenderer) {
        renderer.draw_sprite(self.sprite, self.transform, vec3(1.0, 1.0, 1.0));
    }
}