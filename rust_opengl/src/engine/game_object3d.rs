use glm::{Vec3, vec3, vec2};

use crate::engine::{ model::{Material, Model}, shader::Shader, texture::Texture, transform::Transform, vertex::Vertex};

pub struct GameObject3D {
    model:Model,
    id:usize,
}

impl GameObject3D {
    pub fn new(model:Model) -> Self {
     
        Self {
            model:model,
            id:0
        }
   }

   pub fn add_instance(&mut self, t:Transform) {
        self.model.add_instance(t);
   }

   pub fn init(&mut self) {
       self.model.create_instances();
   }

   pub fn draw(&self, shader:&Shader) {
       self.model.draw(shader);
   }
}