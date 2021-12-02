use core::f32;

use crate::glm::{Vec3, vec3, vec2};
use super::resource_manager::ResourceManager;
use super::shader::Shader;
use super::input_manager::InputManager;
use super::transform::Transform;
use super::model::Model;

pub struct Player {
    model:Model,
    transform:Transform,
    position:Vec3,
    forward:Vec3,
    speed:f32,
    max_speed:f32,
    turn_speed:f32,
    y_vel:f32
}

impl Player {
    pub fn new(transform:Transform) -> Self {
        
        Self {
            model:ResourceManager::get_instance().get_model("player"),
            transform:transform,
            position:transform.translation,
            forward: vec3(0.0, 0.0, 1.0),
            speed: 0.0,
            max_speed: 10.0,
            turn_speed: 20.0,
            y_vel: 0.0
        }
   }

   pub fn init(&mut self) {
        self.model.add_instance(self.transform);
        self.model.create_instances();
   }

   
    pub fn render(&self, shader:&Shader) {
        shader.set_matrix4("model", &self.transform.model_matrix);
        self.model.draw(shader);
   }

   pub fn update(&mut self, dt:f32) {
        //self.y_vel -= 9.8 * dt;
        
        self.process_input(dt);
        self.position += self.forward * self.speed * dt;
        self.position.y += self.y_vel * dt;
        self.transform.update_translation(self.position);
        self.model.update_instance(0, self.transform);
   }

   pub fn process_input(&mut self, dt:f32) {
        let mut movement = InputManager::get_instance().get_movement_input();
    
        if InputManager::get_instance().is_gamepad() {
            movement.z = InputManager::get_instance().get_gamepad_right_trigger() - InputManager::get_instance().get_gamepad_left_trigger();
        } 
        if movement.z > f32::EPSILON {
            self.speed += 5.0 * movement.z * dt;
            if self.speed > self.max_speed {
                self.speed = self.max_speed;
            }
        }  else if movement.z < -f32::EPSILON {
            self.speed -= 5.0 * dt;
            if self.speed < -self.max_speed {
                self.speed = -self.max_speed;
            }
        } else if self.speed != 0.0 {
            self.speed *= 1.0 - dt;
            if self.speed.abs() < 0.25 {
                self.speed = 0.0;
            }
        }
        

         if movement.x.abs() > f32::EPSILON {
            let rotation = self.turn_speed.to_radians() * movement.x * dt;
            self.transform.rotate(vec3(0.0, rotation, 0.0));
            let fwd = glm::rotate_vec3(&self.forward, rotation, &vec3(0.0, 1.0, 0.0));
            self.forward = fwd;
        }
   }

   pub fn get_transform(&self) -> &Transform {
       &self.transform
   }
}