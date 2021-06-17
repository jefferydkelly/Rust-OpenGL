use std::path::{Path, PathBuf};

use crate::{mesh::Mesh, shader::Shader, vertex::Vertex, texture::Texture};
use tobj;
use glm::{Vec2, vec2, Vec3, vec3, Mat4};

#[derive(Clone, Copy)]
pub struct Material {
    pub ambient:Vec3,
    pub diffuse:Vec3,
    pub specular:Vec3,
    pub shininess:f32
}
pub struct Model {
    meshes:Vec<Mesh>,
    materials:Vec<Material>,
    texture:Texture
}

impl Model {
    pub fn new(path:&str, tex:Texture) -> Self {
       let new_path = Path::new(path);
       let cannon = new_path.canonicalize().unwrap();
       let mut path_str = cannon.to_str().unwrap();
       if path_str.starts_with("//?/") {
           let (_, act_path) = path_str.split_at(4);
           path_str = act_path;
       }
       println!("{}", &path_str);
        let obj = tobj::load_obj(path_str,
             &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
             });
        
        

        let (models, materials) = obj.expect("Failed to load OBJ file");
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
            let mut net = Mesh::new(vertices, indy);
            mesh_vec.push(net);
        }

        let materials = materials.expect("Failed to load MTL file");
        let mut mat_vec:Vec<Material> = Vec::new();
        for (i, m) in materials.iter().enumerate() {
            let mat = Material{
                ambient: vec3(m.ambient[0], m.ambient[1], m.ambient[2]),
                diffuse: vec3(m.diffuse[0], m.diffuse[1], m.diffuse[2]),
                specular: vec3(m.specular[0], m.specular[1], m.specular[2]),
                shininess: m.shininess
            };
            mat_vec.push(mat);
        }
       

        Self {
            meshes:mesh_vec,
            materials:mat_vec,
            texture:tex
        }
   }

   pub fn draw(&self, shader:&mut Shader) {
       unsafe {
       shader.use_program();
       gl::ActiveTexture(gl::TEXTURE_2D);
       self.texture.bind();
       for mesh in self.meshes.iter() {
           mesh.draw(shader)
       }
    }
   }

   pub fn add_instance(&mut self, mat:Mat4) {
       for i in 0..self.meshes.len() {
           self.meshes[i].add_instance(mat);
       }
   }

   pub fn create_instances(&mut self) {
    for i in 0..self.meshes.len() {
        self.meshes[i].create_instances();
    }
   }
}