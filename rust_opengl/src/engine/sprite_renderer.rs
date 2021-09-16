use crate::engine::shader::Shader;
use crate::engine::texture::Texture;
use crate::engine::transform;
use crate::engine::transform2d::Transform2D;

extern crate gl;
use gl::{BindBuffer, types::*};
use glm::vec3;

extern crate nalgebra_glm as glm;

use core::f32;
use std::{ffi::c_void, mem, ptr};

#[derive(Debug)]
pub struct SpriteRenderer {

    shader:Shader,
    quad_vao:GLuint
}

impl SpriteRenderer {
    pub fn new(width:f32, height:f32, shady:Shader) -> SpriteRenderer {

        let projection = glm::ortho(0.0, width as f32, 0.0, height as f32, -1.0, 1.0);
        shady.use_program();
        shady.set_matrix4("projection", &projection);
        shady.set_int("sprite", 0);
        SpriteRenderer {
            shader:shady,
            quad_vao: 1,
        }

    }

    pub fn init_render_data(&mut self) {

        self.shader.use_program();
        self.shader.set_int("text", 0);
        self.shader.set_vector3f("textColor", 1.0, 1.0, 1.0);

        unsafe {
            
            let vertices:[f32; 24] = [
                0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.0,

                0.0, 1.0, 0.0, 1.0,
                1.0, 0.0, 1.0, 0.0,
                1.0, 1.0, 1.0, 1.0
            ];

           

            
            let (mut VBO,mut VAO) = (0,0);
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);
 
            
            gl::BindVertexArray(VAO);

            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER, 
                            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                            &vertices[0] as *const f32 as *const c_void,
                            gl::STATIC_DRAW);
            
           
            
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(0);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            self.quad_vao = VAO;
        }
    }

    pub fn draw_sprite(&self, texture:Texture, transform:Transform2D, color:glm::Vec3) {
        unsafe  {
            self.shader.use_program();
            
            let model = transform.model_matrix;

            self.shader.set_matrix4("model", &model);
          
            self.shader.set_vector3f_glm("spriteColor", color);
         

        

            gl::BindVertexArray(self.quad_vao);
            gl::ActiveTexture(gl::TEXTURE0);
            texture.bind();
            
            
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        } 
    }
}