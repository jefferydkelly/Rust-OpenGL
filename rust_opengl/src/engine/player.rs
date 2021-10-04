use core::f32;
use std::path::Path;

use crate::enemy::Enemy;
use crate::glm::{Vec3, vec3, vec2};
use super::physics::Physics;
use super::shader::Shader;
use super::input_manager::InputManager;
use super::aabb::AABB;
use super::transform::Transform;
use super::mesh::Mesh;
use super::vertex::Vertex;
use super::model::{Material, Model};

pub struct Player {
    meshes:Vec<Mesh>,
    material: Material,
    shader:Shader,
    transform:Transform,
    position:Vec3,
    collider:usize
}

impl Player {
    pub fn new(path:&str, shader:Shader, mat:Material, transform:Transform) -> Self {
    
        let new_path = Path::new(path);
       let cannon = new_path.canonicalize().unwrap();
       let path_str = cannon.to_str().unwrap();
      
        let obj = tobj::load_obj(path_str,
             &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
             });
        
        

        let (models, _) = obj.expect("Failed to load OBJ file");
        let mut mesh_vec: Vec<Mesh> = Vec::new();
        let mut max_pos = vec3(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        let mut min_pos = vec3(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        for (_, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
   
            let mut vertices:Vec<Vertex> = Vec::new();

            for v in 0..mesh.positions.len() / 3 {
                let mut start = v * 3;
                let pos = vec3(mesh.positions[start], mesh.positions[start + 1], mesh.positions[start + 2]);

                if pos.x > max_pos.x {
                    max_pos.x = pos.x;
                }

                if pos.y > max_pos.y {
                    max_pos.y = pos.y;
                }

                if pos.z > max_pos.z {
                    max_pos.z = pos.z;
                }

                if pos.x < min_pos.x {
                    min_pos.x = pos.x;
                }

                if pos.y < min_pos.y {
                    min_pos.y = pos.y;
                }

                if pos.z < min_pos.z {
                    min_pos.z = pos.z;
                }
                let norm = vec3(mesh.normals[start], mesh.normals[start + 1], mesh.normals[start + 2]);

                start = v * 2;
                let tex = vec2(mesh.texcoords[start], 1.0 - mesh.texcoords[start + 1]);

                let vert = Vertex::new(pos, norm, tex);
                vertices.push(vert);
            }

            let indy:Vec<u32> = mesh.indices.to_owned();
            let net = Mesh::new(vertices, indy);
            mesh_vec.push(net);
        }
    
        let id = Physics::get_instance().add_body(transform, min_pos, max_pos);
        
        Self {
            meshes:mesh_vec,
            material:mat,
            shader:shader,
            transform:transform,
            position:transform.translation,
            collider:id
        }
   }

   pub fn draw(&self) {
        self.shader.use_program();
        self.shader.set_material("material", self.material);
        self.shader.set_matrix4("model", &self.transform.model_matrix);
        self.material.bind();
        
        for mesh in self.meshes.iter() {
            mesh.draw();
        }
    
   }

   pub fn process_input(&mut self, dt:f32) {
       let col = Physics::get_instance().check_for_collision(self.collider);
       if col.is_some() {
            let my_body = Physics::get_instance().get_body(self.collider);
            let (imax, imin) = my_body.get_max_and_min_points();
            let other  = col.unwrap();
            let (omax, omin) = other.get_max_and_min_points();
          
            let my_ext = my_body.get_extents();
            let total_ext = my_ext + other.get_extents();
            let dif = other.get_center() - my_body.get_center();
            
            let ax = dif.x.abs();
            let ay = dif.y.abs();
            let az = dif.z.abs();

            let ab_dif = vec3(ax, ay, az);

            let overlap = total_ext - ab_dif;
             
            let mut correction = Vec3::zeros();
            if ax < total_ext.x && ax >= az {
                correction.x = -(dif.x/ax) * (overlap.x + f32::EPSILON);
            } 
            
            /* 
            if ay < total_ext.y && ay >= ax && ay >= az {
                correction.y = -(dif.y/ay)  * (overlap.y + f32::EPSILON);
            } */
            
            if az < total_ext.z && az >= ax {
                correction.z = -(dif.z/az)  * (overlap.z + f32::EPSILON);
            }
            
           self.position += correction;
       }
        
       let movement = InputManager::get_instance().get_movement_input();

        if  InputManager::get_instance().get_key_state(glfw::Key::Q) {
            self.transform.rotate(vec3(0.0, -dt, 0.0));
        } 

        if  InputManager::get_instance().get_key_state(glfw::Key::E) {
            self.transform.rotate(vec3( dt, 0.0, 0.0));
        }

        if  InputManager::get_instance().get_key_state(glfw::Key::R) {
            self.transform.reset_rotation();
        }

        

        self.position += movement * dt * 10.0;
        self.transform.update_translation(self.position);
        Physics::get_instance().update_body(self.collider, self.transform);
   }

   pub fn get_transform(&self) -> &Transform {
       &self.transform
   }
}