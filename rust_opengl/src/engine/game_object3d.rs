use glm::{Vec3, vec3, vec2};

use crate::engine::{mesh::Mesh, model::{Material, Model}, shader::Shader, texture::Texture, transform::Transform, vertex::Vertex};
use rapier3d::prelude::*;
use tobj;

pub struct GameObject3D {
    model:Model,
    id:usize,
    transform:Transform
}

impl GameObject3D {
    pub fn new(model:&Model, transform:Transform) -> Self {
     
        Self {
            model:model.to_owned(),
            transform:transform,
            id:0
        }
   }

   pub fn init(&mut self) {
       self.id = self.model.add_instance(self.transform);
   }

   pub fn update(&mut self) {
        self.model.update_instance(self.id, self.transform);
   }

   pub fn draw(&self) {
       //self.model.draw();
   }
}