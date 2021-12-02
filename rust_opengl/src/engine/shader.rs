extern crate gl;
use gl::types::*;
use na::storage::CStride;
use core::f32;
use std::os::raw::c_char;
use std::{ffi::CString, ptr, str};
extern crate nalgebra_glm as glm;
use crate::engine::lights::*;
use crate::engine::model::Material;

#[derive(Clone, Copy, Debug)]
pub  struct Shader {
    id:u32
}

impl Shader {

    /*
    Creates a new Shader object
    return - A new Shader
    */
    pub fn new()->Self {
        Self {id:0}
    }

    /*
    Sets the Shader as the currently active shader program
    return - a pointer to this shader
    */
    pub fn use_program(&self)->&Shader {
        unsafe { 
            gl::UseProgram(self.id); 
        }
        self
    }

    /*
    Compiles the vertex and fragment shader from the code passed in and reports any compilation errors to the user
    vertex_shader_source - The text of the vertex shader code
    fragment_shader_source - The text of the fragment shader code
    */
    pub fn compile(&mut self, vertex_shader_source:String, fragment_shader_source:String) {
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

     /*
    Compiles the vertex and fragment shader from the code passed in and reports any compilation errors to the user
    vertex_shader_source - The text of the vertex shader code
    fragment_shader_source - The text of the fragment shader code
    */
    pub fn compile_with_geometry(&mut self, vertex_shader_source:&str, fragment_shader_source:&str, geometry_shader_source:&str) {
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            self.check_compile_errors(vertex_shader, "Vertex");
            

            let geometry_shader = gl::CreateShader(gl::GEOMETRY_SHADER);
            let c_str_geo = CString::new(geometry_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(geometry_shader, 1, &c_str_geo.as_ptr(), ptr::null());
            gl::CompileShader(geometry_shader);
            self.check_compile_errors(geometry_shader, "Geometry");

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            self.check_compile_errors(fragment_shader, "Fragment");

            self.id = gl::CreateProgram();
            gl::AttachShader(self.id, vertex_shader);
            gl::AttachShader(self.id, geometry_shader);
            gl::AttachShader(self.id, fragment_shader);
            gl::LinkProgram(self.id);
            self.check_compile_errors(self.id, "Program");

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(geometry_shader);
            gl::DeleteShader(fragment_shader);
        }

    }

    /*
    Checks for compilation errors and prints out any it finds
    object - The id of this shader
    comp_type - The kind of shader that is being compiled (Fragment, Vertex or the full Program)
    */
    fn check_compile_errors(&self, object:u32, comp_type:&str) {
        unsafe {
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);

            if comp_type != "Program" {
                gl::GetShaderiv(object, gl::COMPILE_STATUS, &mut success);

                if success != gl::TRUE as GLint {
                    gl::GetShaderInfoLog(object, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                    println!("ERROR::SHADER::{}::COMPILATION_FAILED\n{}", comp_type, str::from_utf8(&info_log).unwrap());
                }
                
            } else {
                gl::GetProgramiv(object, gl::LINK_STATUS, &mut success);
             
                if success != gl::TRUE as GLint {
                    gl::GetProgramInfoLog(object, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                    println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
                }
             }
        }

    }

    /*
    Sets the value of a uniform float value in the shader
    name - The name of the uniform float value
    value - The value to which it will be set
    */
    pub fn set_float(&self, name:&str, value:f32) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform1f(gl::GetUniformLocation(self.id, cs.as_ptr()), value);
          
        }
    }

    /*
    Sets the value of a uniform int value in the shader
    name - The name of the uniform int value
    value - The value to which it will be set
    */
    pub fn set_int(&self, name:&str, value:i32) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform1i(gl::GetUniformLocation(self.id, cs.as_ptr()), value);
        }
    }

    /*
    Sets the value of a uniform vec2 value in the shader
    name - The name of the uniform vec2 value
    x - The x value of the vector
    y - The y value of the vector
    */
    pub fn set_vector2f(&self, name:&str, x:f32, y:f32) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform2f(gl::GetUniformLocation(self.id, cs.as_ptr()), x, y);
        }
    }

    /*
    Sets the value of a uniform vec2 value in the shader
    name - The name of the uniform vec2 value
    value - A Vec2 whose values will be copied to the uniform value
    */
    pub fn set_vector2f_glm(&self, name:&str, value:glm::Vec2) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform2f(gl::GetUniformLocation(self.id, cs.as_ptr()), value.x, value.y);
        }
    }

     /*
    Sets the value of a uniform vec3 value in the shader
    name - The name of the uniform vec3 value
    x - The x value of the vector
    y - The y value of the vector
    z - The z value of the vector
    */
    pub fn set_vector3f(&self, name:&str, x:f32, y:f32, z:f32) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform3f(gl::GetUniformLocation(self.id, cs.as_ptr()), x, y, z);
        }
    }

     /*
    Sets the value of a uniform vec3 value in the shader
    name - The name of the uniform vec3 value
    value - A Vec3 whose values will be copied to the uniform value
    */
    pub fn set_vector3f_glm(&self, name:&str, value:glm::Vec3) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform3f(gl::GetUniformLocation(self.id, cs.as_ptr()), value.x, value.y, value.z);
           
        }
    }

     /*
    Sets the value of a uniform vec4 value in the shader
    name - The name of the uniform vec4 value
    x - The x value of the vector
    y - The y value of the vector
    z - The z value of the vector
    w - The w value of the vector
    */
    pub fn set_vector4f(&self, name:&str, x:f32, y:f32, z:f32, w:f32) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform4f(gl::GetUniformLocation(self.id, cs.as_ptr()), x, y, z, w);
        }
    }

    /*
    Sets the value of a uniform vec4 value in the shader
    name - The name of the uniform vec4 value
    value - A Vec4 whose values will be copied to the uniform value
    */
    pub fn set_vector4f_glm(&self, name:&str, value:glm::Vec4) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::Uniform4f(gl::GetUniformLocation(self.id, cs.as_ptr()), value.x, value.y, value.z, value.w);
        }
    }

    /*
    Sets the value of the uniform mat4 value of the given name in shader
    name - The name of the uniform mat4 value
    matrix - A reference to the matrix whose values will be coped to the uniform value
    */
    pub  fn set_matrix4(&self, name:&str, matrix: &glm::Mat4) {
        unsafe {
            let cs = CString::new(name).unwrap();
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, cs.as_ptr()),1, gl::FALSE, matrix.as_ptr());
        }
    }

    /*
    Sets the value of the Dirctional Light in the shader
    name - The name of the uniform value
    value - The Directional Light whose information will be used
    */
    pub fn set_dir_light(&self, name:&str, value:DirectionalLight) {
        self.set_vector3f_glm(&format!("{}{}", name,".direction"),value.direction);
        self.set_vector3f_glm(&format!("{}{}", name,".ambient"), value.ambient);
        self.set_vector3f_glm(&format!("{}{}", name,".diffuse"), value.diffuse);
        self.set_vector3f_glm(&format!("{}{}", name,".specular"), value.specular);
    }

    /*
    Sets the value of the Point Light in the shader
    name - The name of the uniform value
    value - The Point Light whose information will be used
    */
    pub fn set_point_light(&self, name:&str, value:PointLight) {
        self.set_vector3f_glm(&format!("{}{}", name,".position"),value.position);

        self.set_vector3f_glm(&format!("{}{}", name,".ambient"), value.ambient);
        self.set_vector3f_glm(&format!("{}{}", name,".diffuse"), value.diffuse);
        self.set_vector3f_glm(&format!("{}{}", name,".specular"), value.specular);

        self.set_float(&format!("{}{}", name,".constant"), value.constant);
        self.set_float(&format!("{}{}", name,".linear"), value.linear);
        self.set_float(&format!("{}{}", name,".quadratic"), value.quadratic);
    }

    /*
    Sets the value of the Spotlight in the shader
    name - The name of the uniform value
    value - The Spotlight whose information will be used
    */
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

    /*
    Sets the value of the Material in the shader
    name - The name of the uniform value
    value - The Material whose information will be used
    */
    pub fn set_material(&self, name:&str, value:Material) {
        self.set_int(&format!("{}{}", name,".diffuse"), 1);
        self.set_int(&format!("{}{}", name,".specular"), 2);
        self.set_float(&format!("{}{}", name,".shininess"), value.shininess);
    }

    pub fn set_uniform_block(&self, name:&str, loc:u32) {
        unsafe {
            let cs = CString::new(name).unwrap();
            let ind:u32 = gl::GetUniformBlockIndex(self.id, cs.as_ptr());
            gl::UniformBlockBinding(self.id, ind, loc);
        }
    }
}


