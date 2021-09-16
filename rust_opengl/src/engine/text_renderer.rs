extern crate freetype as ft;

use gl::types::*;
use glm::{Vec2, Vec3, Mat4};
use nalgebra_glm::vec2;

use crate::engine::shader::Shader;
use gl;
use core::f32;
use std::collections::HashMap;
use std::ffi::c_void;
use std::mem;

#[derive(Debug)]
pub struct Character {
    texture_id:u32,
    size:Vec2,
    bearing:Vec2,
    advance:u32
}

#[derive(Debug)]
pub struct TextRenderer {
    text_shader:Shader,
    text_vao:u32,
    text_vbo:u32,
    characters: HashMap<char, Character>
}


impl TextRenderer {

    pub fn new(width: u32, height:u32, shader:Shader) -> Self {
        let projection:Mat4 = glm::ortho(0.0, width as f32, 0.0, height as f32, -1.0, 1.0);
        shader.use_program();
        shader.set_matrix4("projection", &projection);
        shader.set_int("text", 0);

        let  (mut vao, mut vbo) = (0u32,0u32);
        unsafe {
        
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (24 * mem::size_of::<GLfloat>()) as GLsizeiptr, std::ptr::null(), gl::DYNAMIC_DRAW);
            
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<GLfloat>() as GLsizei, std::ptr::null());
            
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

        }

        Self {
            text_shader:shader,
            text_vao:vao,
            text_vbo:vbo,
            characters:HashMap::new()
        }
    }

    pub fn load_font(&mut self, path:&str, size:u32) {
        unsafe {
             
            let library = ft::Library::init().unwrap();
            let face = library.new_face(path, 0).unwrap();
            

            face.set_pixel_sizes(0, size).unwrap();
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            for i in 0..128 {
                let c = char::from_u32(i).unwrap();
                face.load_char(i as usize, ft::face::LoadFlag::RENDER).unwrap();
                let glyph = face.glyph();
                
                let bitmap = glyph.bitmap();
                let width = bitmap.width();
                let height = bitmap.rows();
                
                let buffer = bitmap.buffer().as_ptr();

                let mut texture:u32 = 0;
                gl::GenTextures(1, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, 
                                width, height, 0,
                         gl::RED, gl::UNSIGNED_BYTE,
                         buffer as *const c_void);
                
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                
                let character = Character {
                    texture_id:texture,
                    size:vec2(width as f32, height as f32),
                    bearing:vec2(glyph.bitmap_left() as f32, glyph.bitmap_top() as f32),
                    advance:glyph.advance().x as u32
                };

                self.characters.insert(c, character);
            }
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
 
    pub  fn draw_text(&self, text:&str, mut x:f32, y:f32, scale:f32, color:Vec3) {
        unsafe {
            self.text_shader.use_program();
            self.text_shader.set_vector3f_glm("textColor", color);
            self.text_shader.set_int("text", 0);
     
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(self.text_vao);

            for c in text.chars() {
                let ch = self.characters.get(&c).unwrap();
                
                let x_pos = x + ch.bearing.x * scale;
                let y_pos = y - (ch.size.y - ch.bearing.y) * scale;

                let width = ch.size.x * scale;
                let height = ch.size.y * scale;
                
                let vertices:[GLfloat;24] = [
                    x_pos, y_pos + height, 0.0, 0.0,
                    x_pos, y_pos, 0.0, 1.0,
                    x_pos + width, y_pos, 1.0, 1.0,
                    x_pos, y_pos + height, 0.0, 0.0,
                    x_pos + width, y_pos, 1.0, 1.0,
                    x_pos + width, y_pos + height, 1.0, 0.0
                ];

                gl::BindTexture(gl::TEXTURE_2D, ch.texture_id);
                gl::BindBuffer(gl::ARRAY_BUFFER, self.text_vbo);
                gl::BufferSubData(gl::ARRAY_BUFFER, 0, (mem::size_of::<GLfloat>() * vertices.len()) as GLsizeiptr, &vertices[0] as *const f32 as *const c_void);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
                x += (ch.advance >> 6) as f32 * scale;            
            
            }

            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    } 

}