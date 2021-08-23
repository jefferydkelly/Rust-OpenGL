use core::f32;
use std::ffi::c_void;
use std::path::Path;
use std::{collections::HashMap};
use std::fs::{self};
use image::GenericImageView;
use nalgebra_glm::{Vec3, vec3, Mat4};
use once_cell::sync::OnceCell;
use serde_json::Value;

use crate::engine::model::{Material, Model};
use crate::engine::shader::Shader;
use crate::engine::texture::Texture;
use crate::engine::lights::*;
use crate::level::Level;
use crate::engine::transform::Transform;

use super::camera::Camera;

static mut RESOURCE_MANAGER:OnceCell<ResourceManager> = OnceCell::new();

#[derive(Debug)]
pub struct ResourceManager {
    shaders:HashMap<String, Shader>,
    textures:HashMap<String, Texture>
}

impl  ResourceManager {
    
    pub fn create_instance() {
        let many = ResourceManager {
            shaders: HashMap::new(),
            textures: HashMap::new(),
            //audio:Audio::new()
        };

        unsafe {
            RESOURCE_MANAGER.set(many).unwrap();
        }
    }

    pub fn get_instance()->&'static mut ResourceManager {
        unsafe  {
            RESOURCE_MANAGER.get_mut().expect("Resource Manager has not been created")
        }
    }

    pub fn get_shader(&self, name:&str)->&Shader {
        if self.shaders.contains_key(name) {
            return self.shaders.get(name).unwrap();
        }
        panic!("There's no shader by that name!");
    }

    pub fn get_all_shaders(&mut self)->Vec<&Shader> {
        return self.shaders.values().collect()
    }

    pub fn load_shader(&mut self, v_shader_src:&str, f_shader_src:&str, name:&str) -> Shader {
        let shady = self.load_shader_from_file(v_shader_src, f_shader_src);
        self.shaders.insert(name.to_string(), shady);
        shady
    }

    fn load_shader_from_file(&self, v_shader_src:&str, f_shader_src:&str) -> Shader {
        let vertex_code;
        let fragment_code;

        vertex_code = self.read_shader_file(v_shader_src);
        fragment_code = self.read_shader_file(f_shader_src);
    
    
        let mut shader:Shader = Shader::new();
        shader.compile(&vertex_code, &fragment_code);
        shader
    }

    fn read_shader_file(&self, file:&str) -> String {
        let code = fs::read_to_string(file).expect("Unable to load shader");
        code
    }

    pub fn get_texture(&mut self, name:&str) -> &Texture {
        if self.textures.contains_key(name) {
            return self.textures.get(name).unwrap();
        }

        panic!("That texture doesn't exist");
    }

    pub fn load_texture(&mut self, src: &str, name:&str) -> Texture {
        let mut texture = Texture::new();
        texture.generate(src);
        self.textures.insert(name.to_string(), texture);
        texture
    }

    pub fn load_cube_map(&mut self, srcs:Vec<&str>, name:&str) -> u32 {
        let mut cube_id:u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut cube_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, cube_id);
            
            for  i in 0..srcs.len() {
                let src = srcs[i];
                let mut img = image::open(&Path::new(src)).expect("Texture failed to load");
                let format = gl::RGB;

                let data = img.to_rgb8().into_raw();

                gl::TexImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_X + (i as u32), 0, format as i32, img.width() as i32,
                                img.height() as i32, 0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
                
            }

            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
                
                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        
        cube_id
    }

    pub fn load_level(&mut self, path:&str) -> Level {

        let mut level = Level::new();
        let code = fs::read_to_string(path).expect("Unable to load JSON");
        let v:Value = serde_json::from_str(&code).unwrap();
     
        for val in v["shaders"].as_array().unwrap() { 
            self.load_shader(val["vertex"].as_str().unwrap(), val["fragment"].as_str().unwrap(), val["name"].as_str().unwrap());
        }

        for val in v["models"].as_array().unwrap() {
            let shader_name = val["shader"].as_str().unwrap();
            let shader = self.get_shader(shader_name);
            
            let mat = val["material"].as_object().unwrap();
            
            let shin = mat["shininess"].as_f64().unwrap() as f32;
            let material:Material = Material::new(mat["diffuse"].as_str().unwrap(), mat["specular"].as_str().unwrap(), shin);
            
            let mut model = Model::new(val["path"].as_str().unwrap(), shader, material);

            let instances = val["instances"].as_array().unwrap();
            for inst in instances{
                let position = self.parse_vec3(inst["position"].to_owned(), false); 
                let rotation = self.parse_vec3(inst["rotation"].to_owned(), true);
                let scale = self.parse_vec3(inst["scale"].to_owned(), false);
        
                let transform  = Transform::new(position, rotation, scale);
                model.add_instance(transform);
            }
            model.create_instances();
            level.add_model(model);
        }

        for val in v["lights"].as_array().unwrap() {
            if val["type"] == "directional" {
                let light = DirectionalLight{
                    direction: self.parse_vec3(val["direction"].to_owned(), false),
                    ambient: self.parse_vec3(val["ambient"].to_owned(), false),
                    diffuse: self.parse_vec3(val["diffuse"].to_owned(), false),
                    specular: self.parse_vec3(val["specular"].to_owned(), false)
                };

                level.add_directional_light(light);
            } else if val["type"] == "point" {
                let light = PointLight{
                    position: self.parse_vec3(val["position"].to_owned(), false),
                    ambient: self.parse_vec3(val["ambient"].to_owned(), false),
                    diffuse: self.parse_vec3(val["diffuse"].to_owned(), false),
                    specular: self.parse_vec3(val["specular"].to_owned(), false),
                    constant: val["constant"].as_f64().unwrap() as f32,
                    linear: val["linear"].as_f64().unwrap() as f32,
                    quadratic: val["quadratic"].as_f64().unwrap() as f32

                };
                
                level.add_point_light(light);
            } else if val["type"] == "spot" {
                let light = Spotlight {
                    cutoff: val["cutoff"].as_f64().unwrap() as f32,
                    outer_cutoff: val["outerCutoff"].as_f64().unwrap() as f32,
                    direction: self.parse_vec3(val["direction"].to_owned(), false),
                    position: self.parse_vec3(val["position"].to_owned(), false),
                    ambient: self.parse_vec3(val["ambient"].to_owned(), false),
                    diffuse: self.parse_vec3(val["diffuse"].to_owned(), false),
                    specular: self.parse_vec3(val["specular"].to_owned(), false),
                    constant: val["constant"].as_f64().unwrap() as f32,
                    linear: val["linear"].as_f64().unwrap() as f32,
                    quadratic: val["quadratic"].as_f64().unwrap() as f32
                };

                level.add_spotlight(light);
            }
        }
       

        level
       
    }

    fn parse_vec3(&self, val:Value, convert:bool)->Vec3 {
        let x = val["x"].as_f64().unwrap() as f32;
        let y = val["y"].as_f64().unwrap() as f32;
        let z = val["z"].as_f64().unwrap() as f32;
        if convert {
            vec3(x.to_radians(),y.to_radians(),z.to_radians())
        } else {
            vec3(x,y,z)
        }
    }

}