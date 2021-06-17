use crate::shader::Shader;
use crate::texture::Texture;

extern crate gl;
use gl::{BindBuffer, types::*};
use glm::vec3;

extern crate nalgebra_glm as glm;

use std::{ffi::c_void, mem, ptr};

pub struct SpriteRenderer {

    shader:Shader,
    quad_vao:GLuint
}

impl SpriteRenderer {
    pub fn new(shady:&Shader) -> SpriteRenderer {
        SpriteRenderer {
            shader:shady.to_owned(),
            quad_vao: 1
        }

    }

    pub fn init_render_data(&mut self) {
        self.quad_vao = unsafe {
            
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
            VAO
        };
    }

    pub fn draw_sprite(&self, texture:Texture, position: glm::Vec2, size:glm::Vec2, rotate:f32, color:glm::Vec3) {
        unsafe  {
            self.shader.use_program();
            let mut model = glm::Mat4::identity();
            model = glm::translate(&mut model, &glm::vec3(position.x, position.y, 0.0));
            
            /* 
            model = glm::translate(&mut model, &glm::vec2_to_vec3(&size).scale(0.5));
            model = glm::rotate(&mut model, rotate.to_radians(), &glm::vec3(0.0, 0.0, 1.0));
            model = glm::translate(&mut model, &glm::vec2_to_vec3(&size).scale(-0.5));
            */
            model = glm::scale(&mut model, &glm::vec3(size.x, size.y, 1.0));

            self.shader.set_matrix4("model", &model);
            self.shader.set_vector3f_glm("spriteColor", color);

            
            
            gl::ActiveTexture(gl::TEXTURE0);
            texture.bind();
            
            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        } 
    }

    pub fn test_draw(&self, position: glm::Vec2, size:glm::Vec2) {
        unsafe  {
            self.shader.use_program();
            let mut model = glm::Mat4::identity();
            model = glm::translate(&mut model, &vec3(position.x, position.y, 0.0));
            
            /* 
            model = glm::translate(&mut model, &glm::vec2_to_vec3(&size).scale(0.5));
            model = glm::rotate(&mut model, rotate.to_radians(), &glm::vec3(0.0, 0.0, 1.0));
            model = glm::translate(&mut model, &glm::vec2_to_vec3(&size).scale(-0.5));
            */
            model = glm::scale(&mut model, &vec3(size.x, size.y, 1.0));

            self.shader.set_matrix4("model", &model);
            self.shader.set_vector3f_glm("color", vec3(1.0, 1.0, 1.0));

            
            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        } 
    }
}