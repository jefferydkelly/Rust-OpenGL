use crate::enemy::Enemy;
use crate::glm::{Vec2, vec3, vec2};
use crate::resource_manager::ResourceManager;
use crate::texture::Texture;
use crate::timer::Timer;
use crate::sprite_renderer::SpriteRenderer;
use crate::traits::{Updated, Rendered, Controlled};
use crate::input_manager::InputManager;
use crate::collider::Collider;
use crate::game_object::GameObject;

use glfw::Key;
pub struct Player {
    position:Vec2,
    velocity:Vec2,
    size:Vec2,
    sprite: Texture,
    timer:Timer,
    pub collider:Collider,
    bullets:Vec<GameObject>,
}

impl Player {

    pub fn new(pos:Vec2, scale:Vec2, spr:Texture, bullet_spr:Texture) -> Self {

        let mut lasers:Vec<GameObject> = Vec::new();
        for i in 0..10 {
            let mut laser = GameObject::new(vec2(0.0, 0.0), vec2(16.0, 32.0), 0.0, bullet_spr);
            laser.set_velocity(vec2(0.0, -300.0));
            laser.set_visibility(false);
            lasers.push(laser);
        }
        Self {
            position: pos,
            size:scale,
            sprite:spr,
            velocity:vec2(0.0, 0.0),
            timer: Timer::new(),
            collider: Collider::new(pos, scale),
            bullets:lasers,
        }
    }

    pub fn init(&mut self) {
        self.timer.start_oneshot(0.25);
    }

    pub fn get_position(&self)->Vec2 {
        self.position
    }

    pub fn get_size(&self)-> Vec2 {
        self.size
    }

    pub fn set_position(&mut self, pos:Vec2) {
        self.position = pos;
        self.collider.position = pos;
    }

    pub fn fire(&mut self) {
        if self.timer.is_ready() {
            for i in 0 .. self.bullets.len() {
                if !self.bullets[i].is_visible() {
                    self.bullets[i].set_position(self.position + vec2(24.0, -50.0));
                    self.bullets[i].set_visibility(true);
                    break;
                }
            }
            self.timer.reset();
        }
    }

    pub fn check_collisions(&mut self, enemy:&mut Enemy) {
        if enemy.is_visible() {
            if self.collider.check_collision(enemy.collider) {
                enemy.set_visibility(false);
                return;
            }

            for b in &mut self.bullets {
                if b.is_visible() && b.collider.check_collision(enemy.collider) {
                    b.set_visibility(false);
                    enemy.set_visibility(false);
                    return;
                }
            }
        }

        
    }
}

impl Updated for Player {
    fn update(&mut self, dt:f32) {
        self.position += self.velocity.scale(dt);
        self.collider.position = self.position;
        self.timer.update(dt);

        for i in 0 .. self.bullets.len() {
            self.bullets[i].update(dt);
        }
    }
}

impl Rendered for Player {
    fn render(&self, renderer:&SpriteRenderer) {
        renderer.draw_sprite(self.sprite, self.position, self.size, 0.0, vec3(1.0, 1.0, 1.0));
    
        for i in 0 .. self.bullets.len() {
            self.bullets[i].draw_sprite(renderer);
        }
    }
}

impl Controlled for Player {
    fn receive_input(&mut self, manager:&InputManager) {
        self.velocity = vec2(0.0, 0.0);

        if manager.get_key_state(Key::A) {
            self.velocity.x = -100.0;
        } else if manager.get_key_state(Key::D) {
            self.velocity.x = 100.0;
        }

        if manager.get_key_state(Key::Space) {
            self.fire();
        }
    }
}