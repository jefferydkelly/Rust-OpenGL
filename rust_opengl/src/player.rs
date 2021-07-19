use core::f32;

use crate::enemy::Enemy;
use crate::glm::{Vec3, vec3};
use crate::engine::transform2d::{Transform2D};
use crate::engine::texture::Texture;
use crate::engine::timer::*;
use crate::engine::sprite_renderer::SpriteRenderer;
use crate::engine::traits::{Updated, Rendered};
use crate::engine::input_manager::InputManager;
use crate::engine::collider::Collider;
use crate::engine::game_object::GameObject;

use glfw::Key;

pub struct Player {
    transform:Transform2D,
    velocity:Vec3,
    sprite: Texture,
    can_fire:bool,
    pub collider:Collider,
    timer_state:TimerState,
    timer_reps: i32,
    run_reps: i32,
    timer_tick: f32,
    timer_cur_time:f32
}

impl Player {

    pub fn new(pos:Vec3, scale:Vec3, spr:Texture) -> Player {
        let t = Transform2D::new(pos, 0.0, scale);
        
        let mut playa = Player {
            transform:t,
            sprite:spr,
            velocity:vec3(0.0, 0.0, 0.0),
            can_fire: true,
            collider: Collider::new(pos, scale),
            timer_state: TimerState::READY,
            timer_reps: 0,
            run_reps: 0,
            timer_tick: 0.0,
            timer_cur_time: 0.0
        };

        playa
    }

    fn reset(&mut self) {
        self.can_fire = true;
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
    
    pub fn fire(&mut self) {
        if self.can_fire {
            self.can_fire = false;
            self.start_timer(2.0, 1);
            println!("Fire!");
        }
    }

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
            self.transform.rotate(dt);
        } else if InputManager::instance().get_key_state(glfw::Key::D) {
            self.transform.rotate(-dt);
        }
        
        if InputManager::instance().get_key_state(glfw::Key::Space) {
            self.fire();
        }

        let rotation = self.transform.get_rotation() - 90.0;
        let sin = rotation.sin();
        let cos = rotation.cos();
        let cen = vec3(400.0, 300.0, 0.0);
        self.transform.update_translation(cen + vec3(cos, sin, 0.0) * 300.0);
        self.update_timer(dt);
    }
}

impl Rendered for Player {
    fn render(&self, renderer:&SpriteRenderer) {
        renderer.draw_sprite(self.sprite, self.transform, vec3(1.0, 1.0, 1.0));
    }
}

impl Timed for Player {
    fn on_tick(&mut self) {
        self.can_fire = true;
    }

    fn on_complete(&mut self) {
        self.can_fire = true;
        self.timer_state = TimerState::READY;
    }

    fn update_timer(&mut self, dt:f32) {
        if self.timer_state == TimerState::RUNNING {
            self.timer_cur_time += dt;
            if self.timer_cur_time >= self.timer_tick {
                self.run_reps += 1;
                if self.run_reps >= self.timer_reps {
                    println!("I can fire again");
                    self.on_complete();
                } else {
                    self.on_tick();
                    self.timer_cur_time -= self.timer_tick;
                }
            }
        }
    }

    fn start_timer(&mut self, time:f32, reps:i32) {
        if self.timer_state == TimerState::READY {
            self.timer_state = TimerState::RUNNING;
            self.timer_reps = reps;
            self.run_reps = 0;
            self.timer_tick = time;
            self.timer_cur_time = 0.0;
        }
    }


}