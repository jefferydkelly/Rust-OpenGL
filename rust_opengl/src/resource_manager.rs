use core::f32;
use std::{collections::HashMap, ffi::CString};
use std::fs::{self, File};
use std::io::BufReader;
use std::time::Duration;
use nalgebra::Matrix5;
use nalgebra_glm::{Vec3, vec3, Mat4};
use serde_json::{Result, Value};

use crate::model::Model;
use crate::shader::Shader;
use crate::texture::Texture;

extern crate rusty_audio;
use rusty_audio::Audio;
pub struct ResourceManager {
    shaders:HashMap<String, Shader>,
    textures:HashMap<String, Texture>,
    audio:Audio
}

impl  ResourceManager {
    
    pub fn new()->Self {
      
       
        Self {
            shaders: HashMap::new(),
            textures: HashMap::new(),
            audio:Audio::new()
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
        //self.shaders.insert(name.to_string(), shady);
        //self.shaders.get(name).unwrap()
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
        //self.textures.insert(name.to_string(), texture);
        
        //&self.textures.get(name).unwrap()
        texture
    }

    pub fn load_sound(&mut self, src: &str, name:&'static str) {
        self.audio.add(name, src);
    }

    pub fn play_sound(&mut self, name:&str) {
        self.audio.play(name);
    }

    pub fn load_json(&mut self, path:&str) -> Vec<Model> {
        let code = fs::read_to_string(path).expect("Unable to load JSON");
        let v:Value = serde_json::from_str(&code).unwrap();
        let mut models:Vec<Model> = Vec::new();

    
        for val in v["models"].as_array().unwrap() {
            
            let texture = self.load_texture(val["texture"].as_str().unwrap(), val["name"].as_str().unwrap());
            let mut model = Model::new(val["path"].as_str().unwrap(), texture);

            let instances = val["instances"].as_array().unwrap();
            for inst in instances{
                let position = self.parse_vec3(inst["position"].to_owned(), false); 
                let rotation = self.parse_vec3(inst["rotation"].to_owned(), true);
                let scale = self.parse_vec3(inst["scale"].to_owned(), false);
        
                let mat = self.create_mat4(position, rotation, scale);
                model.add_instance(mat);
            }
            model.create_instances();
            models.push(model);
        }

        models
       
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

    fn create_mat4(&self, position:Vec3, rotation:Vec3, scale:Vec3) -> Mat4 {
        let mut model = Mat4::identity();
        model = glm::translate(&model, &position);
        model = glm::rotate(&model, rotation.y, &vec3(0.0, 1.0, 0.0));
        model = glm::scale(&model, &scale);

        model
    }

}