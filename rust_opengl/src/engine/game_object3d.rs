use glm::{Vec3, vec3, vec2};

use crate::engine::{mesh::Mesh, model::{Material, Model}, physics_manager::PhysicsManager, shader::Shader, texture::Texture, transform::Transform, vertex::Vertex};
use rapier3d::prelude::*;
use tobj;

pub struct GameObject3D {
    model:Model,
    id:usize,
    transform:Transform,
    body_handle: RigidBodyHandle,
    collider_handle: ColliderHandle
}

impl GameObject3D {
    pub fn new(model:&Model, transform:Transform) -> Self {
        let bhandle = PhysicsManager::get_instance().create_dynamic_body();
        let chandle = PhysicsManager::get_instance().create_cuboid_collider(transform.translation, transform.scale, bhandle);
        Self {
            model:model.to_owned(),
            transform:transform,
            id:0,
            body_handle: bhandle,
            collider_handle: chandle
        }
   }

   pub fn init(&mut self) {
       self.id = self.model.add_instance(self.transform);
   }

   pub fn update(&mut self) {
        let body = PhysicsManager::get_instance().get_rigid_body(self.body_handle);
        self.transform.update_translation(body.position().translation.vector);
        self.model.update_instance(self.id, self.transform);
   }

   pub fn draw(&self) {
       //self.model.draw();
   }
}