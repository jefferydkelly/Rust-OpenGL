extern crate gl;
use gl::types::*;
use core::f32;
use std::{ffi::CString, ptr, str};
extern crate nalgebra_glm as glm;
use crate::engine::lights::*;
use crate::engine::model::Material;

#[derive(Clone, Copy, Debug)]
pub  struct Shader {
    id:u32
}

impl Shader {
    pub fn new()->Self {
        Self {id:0}
    }

    pub fn use_program(&self)->&Shader {
        unsafe { 
            gl::UseProgram(self.id); 
        }
        self
    }

    pub fn compile(&mut self, vertex_shader_source:&str, fragment_shader_source:&str) {
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            self.check_compile_errors(vertex_shader, "Vertex");
            

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            self.check_compile_errors(fragment_shader, "Fragment");

            self.id = gl::CreateProgram();
            gl::AttachShader(self.id, vertex_shader);
            gl::AttachShader(self.id, fragment_shader);
            gl::LinkProgram(self.id);
            self.check_compile_errors(self.id, "Program");

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

    }

    fn check_compile_errors(&self, object:u32, comp_type:&str) {
        unsafe {
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(1024);

            if comp_type != "Program" {
          
                gl::GetShaderiv(object, gl::COMPILE_STATUS, &mut success);

                if success != gl::TRUE as GLint {
                    gl::GetShaderInfoLog(object, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                    println!("ERROR::SHADER::{}::COMPILATION_FAILED\n{}", comp_type, str::from_utf8(&info_log).unwrap());
                }
                
            } else {
                gl::GetProgramiv(object, gl::LINK_STATUS, &mut success);

                if success != gl::TRUE as GLint {
                    gl::GetProgramInfoLog(object, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                    println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
                }
             }
        }

    }

    pub fn set_float(&self, name:&str, value:f32) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform1f(gl::GetUniformLocation(self.id, cs.as_ptr()), value);
          
        }
    }

    pub fn set_int(&self, name:&str, value:i32) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform1i(gl::GetUniformLocation(self.id, cs.as_ptr()), value);
        }
    }

    pub fn set_vector2f(&self, name:&str, x:f32, y:f32) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform2f(gl::GetUniformLocation(self.id, cs.as_ptr()), x, y);
        }
    }

    pub fn set_vector2f_glm(&self, name:&str, value:glm::Vec2) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform2f(gl::GetUniformLocation(self.id, cs.as_ptr()), value.x, value.y);
        }
    }

    pub fn set_vector3f(&self, name:&str, x:f32, y:f32, z:f32) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform3f(gl::GetUniformLocation(self.id, cs.as_ptr()), x, y, z);
        }
    }

    pub fn set_vector3f_glm(&self, name:&str, value:glm::Vec3) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform3f(gl::GetUniformLocation(self.id, cs.as_ptr()), value.x, value.y, value.z);
           
        }
    }

    pub fn set_vector4f(&self, name:&str, x:f32, y:f32, z:f32, w:f32) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform4f(gl::GetUniformLocation(self.id, cs.as_ptr()), x, y, z, w);
        }
    }

    pub fn set_vector4f_glm(&self, name:&str, value:glm::Vec4) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform4f(gl::GetUniformLocation(self.id, cs.as_ptr()), value.x, value.y, value.z, value.w);
        }
    }

    pub  fn set_matrix4(&self, name:&str, matrix: &glm::Mat4) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, cs.as_ptr()),1, gl::FALSE, matrix.as_ptr());
        }
    }

    pub fn set_dir_light(&self, name:&str, value:DirectionalLight) {
        self.set_vector3f_glm(&format!("{}{}", name,".direction"),value.direction);
        self.set_vector3f_glm(&format!("{}{}", name,".ambient"), value.ambient);
        self.set_vector3f_glm(&format!("{}{}", name,".diffuse"), value.diffuse);
        self.set_vector3f_glm(&format!("{}{}", name,".specular"), value.specular);
    }

    pub fn set_point_light(&self, name:&str, value:PointLight) {
        self.set_vector3f_glm(&format!("{}{}", name,".position"),value.position);

        self.set_vector3f_glm(&format!("{}{}", name,".ambient"), value.ambient);
        self.set_vector3f_glm(&format!("{}{}", name,".diffuse"), value.diffuse);
        self.set_vector3f_glm(&format!("{}{}", name,".specular"), value.specular);

        self.set_float(&format!("{}{}", name,".constant"), value.constant);
        self.set_float(&format!("{}{}", name,".linear"), value.linear);
        self.set_float(&format!("{}{}", name,".quadratic"), value.quadratic);
    }

    pub fn set_spotlight(&self, name:&str, value:Spotlight) {
        self.set_vector3f_glm(&format!("{}{}", name,".direction"),value.direction);
        self.set_vector3f_glm(&format!("{}{}", name,".position"),value.position);

        self.set_vector3f_glm(&format!("{}{}", name,".ambient"), value.ambient);
        self.set_vector3f_glm(&format!("{}{}", name,".diffuse"), value.diffuse);
        self.set_vector3f_glm(&format!("{}{}", name,".specular"), value.specular);

        self.set_float(&format!("{}{}", name,".constant"), value.constant);
        self.set_float(&format!("{}{}", name,".linear"), value.linear);
        self.set_float(&format!("{}{}", name,".quadratic"), value.quadratic);

        self.set_float(&format!("{}{}", name,".cutoff"), value.cutoff.to_radians().cos());
        self.set_float(&format!("{}{}", name,".outerCutoff"), value.outer_cutoff.to_radians().cos());
    }

    pub fn set_material(&self, name:&str, value:Material) {
        self.set_int(&format!("{}{}", name,".diffuse"), 1);
        self.set_int(&format!("{}{}", name,".specular"), 2);
        self.set_float(&format!("{}{}", name,".shininess"), value.shininess);
    }
}


