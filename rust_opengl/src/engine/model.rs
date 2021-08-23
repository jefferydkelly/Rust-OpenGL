use std::{path::{Path, PathBuf}, usize};

use crate::engine::{mesh::Mesh, shader::Shader, vertex::Vertex, texture::Texture, transform::Transform};
use tobj;
use glm::{Vec2, vec2, Vec3, vec3, Mat4};

use super::resource_manager::ResourceManager;

#[derive(Clone, Copy)]
pub struct Material {
    pub diffuse:Texture,
    pub specular:Texture,
    pub shininess:f32
}

impl Material {

    pub fn new(dif_src:&str, spec_src:&str, shin:f32)-> Self {
        let dif = ResourceManager::get_instance().load_texture(dif_src, "specular");
        let spec = ResourceManager::get_instance().load_texture(spec_src, "specular");
        Self {
            diffuse: dif,
            specular: spec,
            shininess: shin
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE1);
            self.diffuse.bind();

            gl::ActiveTexture(gl::TEXTURE2);
            self.specular.bind();
        }
    }
}

#[derive(Clone)]
pub struct Model {
    meshes:Vec<Mesh>,
    material: Material,
    shader:*const Shader
}

impl Model {
    pub fn new(path:&str, shader:&Shader, mat:Material) -> Self {
    
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
        for (_, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
   
            let mut vertices:Vec<Vertex> = Vec::new();

            for v in 0..mesh.positions.len() / 3 {
                let mut start = v * 3;
                let pos = vec3(mesh.positions[start], mesh.positions[start + 1], mesh.positions[start + 2]);
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
      
        Self {
            meshes:mesh_vec,
            material:mat,
            shader:shader
        }
   }

   pub fn draw(&self, shader:&Shader) {
       unsafe {
        
        shader.set_material("material", self.material);
        self.material.bind();
        
        for mesh in self.meshes.iter() {
            mesh.draw();
        }
    }
   }

   pub fn add_instance(&mut self, t:Transform) -> usize {
       let mut index = 0;
        for i in 0..self.meshes.len() {
            index = self.meshes[i].add_instance(t);
        }
        index
   }

   pub fn update_instance(&mut self, u:usize, t:Transform) {
    for i in 0..self.meshes.len() {
        self.meshes[i].update_instance(u, t);
    }
   }

   pub fn create_instances(&mut self) {
    for i in 0..self.meshes.len() {
        self.meshes[i].create_instances();
    }
   }
}