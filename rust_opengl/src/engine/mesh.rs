extern crate nalgebra_glm as glm;

use std::ffi::c_void;
use std::ptr;
use std::u32;
use std::mem;

use glm::{Mat4, Vec4};
use memoffset::offset_of;
use crate::engine::shader::Shader;
use crate::engine::vertex::Vertex;
use crate::engine::transform::Transform;
use gl;
use gl::types::*;

#[derive(Clone)]
pub struct Mesh {
    pub vao:u32,
    vbo:u32,
    ebo:u32,
    pub vertices:Vec<Vertex>,
    pub indices:Vec<u32>,
    num_instances:i32,
    instances: Vec<Mat4>,
    instance_vbo:u32
}

impl Mesh {
    pub fn new(verts:Vec<Vertex>, inds:Vec<u32>) -> Mesh {
        let mut meshy = Mesh {
            vao:0,
            vbo:0,
            ebo:0,
            vertices:verts,
            indices:inds,
            num_instances:0,
            instances: Vec::new(),
            instance_vbo: 0
        };
        meshy.setup_mesh();
        meshy
    }
 
 
    fn setup_mesh(&mut self) {
        
        unsafe  {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr, &self.vertices[0] as *const Vertex as *const GLvoid, gl::STATIC_DRAW);
        
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (self.indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr, &self.indices[0] as *const u32 as *const GLvoid, gl::STATIC_DRAW);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, std::mem::size_of::<Vertex>() as GLsizei, ptr::null());

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, std::mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, normal) as *const GLvoid);

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, std::mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, tex_coords) as *const GLvoid);

            
            gl::GenBuffers(1, &mut self.instance_vbo);
            
            self.num_instances = 0;

            gl::BindVertexArray(0);
        }
    }

    pub fn add_instance(&mut self, t:Transform)->usize {
        self.instances.push(t.model_matrix);
        self.num_instances+=1;
        return (self.num_instances - 1) as usize;
    }

    pub fn update_instance(&mut self, index:usize, t:Transform) {
        self.instances[index] = t.model_matrix;
    }

    pub fn create_instances(&mut self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instance_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<Mat4>() * self.instances.len()) as GLsizeiptr, self.instances.as_ptr()  as *const GLvoid, gl::STATIC_DRAW);
            

            let mat_size = mem::size_of::<Mat4>() as GLsizei;
            let vec_size = mem::size_of::<Vec4>();
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(3, 4, gl::FLOAT, gl::FALSE, mat_size, ptr::null());

            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(4, 4, gl::FLOAT, gl::FALSE, mat_size, vec_size as *const c_void);

            gl::EnableVertexAttribArray(5);
            gl::VertexAttribPointer(5, 4, gl::FLOAT, gl::FALSE, mat_size, (vec_size * 2) as *const c_void);

            gl::EnableVertexAttribArray(6);
            gl::VertexAttribPointer(6, 4, gl::FLOAT, gl::FALSE, mat_size, (vec_size * 3) as *const c_void);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::VertexAttribDivisor(3,1);
            gl::VertexAttribDivisor(4,1);
            gl::VertexAttribDivisor(5,1);
            gl::VertexAttribDivisor(6,1);
            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElementsInstanced(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null(), self.num_instances);
            gl::BindVertexArray(0);
        }

    }
}