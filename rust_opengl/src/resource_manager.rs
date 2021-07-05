use core::f32;
use std::{collections::HashMap};
use std::fs::{self};
use nalgebra_glm::{Vec3, vec3, Mat4};
use once_cell::sync::OnceCell;
use serde_json::Value;

use crate::model::Model;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::lights::*;
use crate::level::Level;
use crate::transform::Transform;

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

    /*
   
    */

    pub fn load_level(&mut self, path:&str) -> Level {

        let mut level = Level::new();
        let code = fs::read_to_string(path).expect("Unable to load JSON");
        let v:Value = serde_json::from_str(&code).unwrap();
     
        for val in v["models"].as_array().unwrap() {
            
            let texture = self.load_texture(val["texture"].as_str().unwrap(), val["name"].as_str().unwrap());
            let vertex_src = val["shader"]["vertex"].as_str().unwrap();
            let fragment_src = val["shader"]["fragment"].as_str().unwrap();
            let shader_name = val["shader"]["name"].as_str().unwrap();

            let shader = self.load_shader(vertex_src,fragment_src,shader_name);
            let mut model = Model::new(val["path"].as_str().unwrap(), texture, shader);

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