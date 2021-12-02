
extern crate memoffset;
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
extern crate gl;

use std::{ffi::c_void, mem, ptr};

use gl::types::GLsizei;

use super::{resource_manager::ResourceManager};

pub struct Skybox {
    id:u32,
    vao:u32
}

impl Skybox {

    /*
    Creates a new Skybox object from the given images
    srcs - A Vector of strings containing the filepaths for the six faces of the skybox
    */
    pub fn new(id:u32) -> Self {

        let vertices:[f32;108] = [
            -1.0,  1.0, -1.0,
            -1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0,  1.0,

            1.0, -1.0, -1.0,
            1.0, -1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0, -1.0,
            1.0, -1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,

            -1.0,  1.0, -1.0,
            1.0,  1.0, -1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            1.0, -1.0,  1.0
        ];

        let mut the_vao:u32 = 0;
        let mut vbo:u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut the_vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(the_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<f32>() * vertices.len()) as isize, &vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW);
        
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<f32>() as GLsizei, ptr::null());
        }
    
        Skybox {
            id:id,
            vao:the_vao
        }
    }

    /*
    Renders the Skybox
    */
    pub fn render(&self) {
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
            gl::BindVertexArray(self.vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.id);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
            gl::DepthFunc(gl::LESS);
        }
    }
}