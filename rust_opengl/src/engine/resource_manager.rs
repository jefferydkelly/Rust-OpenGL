use core::f32;
use std::ffi::c_void;
use std::hash::Hash;
use std::path::Path;
use std::{collections::HashMap};
use std::thread;
use std::fs::{self};
use image::GenericImageView;
use nalgebra_glm::{Vec3, vec3};
use once_cell::sync::OnceCell;
use serde_json::Value;

use crate::engine::game_object3d::GameObject3D;
use crate::engine::model::{Material, Model};
use crate::engine::shader::Shader;
use crate::engine::texture::Texture;
use crate::engine::lights::*;
use crate::level::{self, Level};
use crate::engine::transform::Transform;

use super::model;
use super::skybox::Skybox;

static mut RESOURCE_MANAGER:OnceCell<ResourceManager> = OnceCell::new();

#[derive(Debug)]
pub struct ResourceManager {
    shaders:HashMap<String, Shader>,
    textures:HashMap<String, Texture>,
    materials:HashMap<String, Material>,
    models:HashMap<String, Model>
}

impl  ResourceManager {
    
    /*
    Creates the single instance of the Resource Manager.
    */
    pub fn create_instance() {
        let many = ResourceManager {
            shaders: HashMap::new(),
            textures: HashMap::new(),
            materials: HashMap::new(),
            models: HashMap::new()
        };

        unsafe {
            RESOURCE_MANAGER.set(many).unwrap();
        }
    }

    /*
    Grants access to the current instance of Resource Manager
    return - The Resource Manager singleton
    */
    pub fn get_instance()->&'static mut ResourceManager {
        unsafe  {
            RESOURCE_MANAGER.get_mut().expect("Resource Manager has not been created")
        }
    }

    /*
    Gets the shader of the passed in name from the ResourceManager if it exists
    name - The name of the shader used as a dictionary key
    return - The shader of that name if it exists in the dictionary
    */
    pub fn get_shader(&mut self, name:&str)->Shader {
        if self.shaders.contains_key(name) {
            return self.shaders.get(name).unwrap().clone();
        }
        panic!("That shader {} doesn't exist", &name);
    }

    /*
    Returns a vector containing all of the shaders currently in the game
    return - A vector of all shaders in the game
    */
    pub fn get_all_shaders(&mut self)->Vec<&Shader> {
        return self.shaders.values().collect()
    }

    /*
    Loads a shader using the given files and inserts it into the Shaders Dictionary using the name as the key
    v_shader_src - The path to the vertex shader file
    f_shader_src - The path to the fragment shader file
    name - The name of the shader to be used as its key in the shaders dictionary
    return - The loaded shader
    */
    pub fn load_shader(&mut self, v_shader_src:&str, f_shader_src:&str, name:&str) -> Shader {
        let shady = self.load_shader_from_file(v_shader_src, f_shader_src);
        self.shaders.insert(name.to_string(), shady);
        shady
    }

    /*
    Loads a shader using the given files and inserts it into the Shaders Dictionary using the name as the key
    v_shader_src - The path to the vertex shader file
    f_shader_src - The path to the fragment shader file
    name - The name of the shader to be used as its key in the shaders dictionary
    return - The loaded shader
    */
    pub fn load_shader_with_geometry(&mut self, v_shader_src:&str, f_shader_src:&str, g_shader_src:&str, name:&str) -> Shader {
        let shady = self.load_shader_from_file_with_geometry(v_shader_src, f_shader_src, g_shader_src);
        self.shaders.insert(name.to_string(), shady);
        shady
    }

    /*
    Loads the shader from the given files and compiles it.
    v_shader_src - The path to the vertex shader file
    f_shader_src - The path to the fragment shader file
    return - The loaded shader
    */
    fn load_shader_from_file(&self, v_shader_src:&str, f_shader_src:&str) -> Shader {
        let vertex_code;
        let fragment_code;

        vertex_code = self.read_shader_file(v_shader_src);
        fragment_code = self.read_shader_file(f_shader_src);
    
    
        let mut shader:Shader = Shader::new();
        shader.compile(vertex_code, fragment_code);
        shader
    }

    /*
    Loads the shader from the given files and compiles it.
    v_shader_src - The path to the vertex shader file
    f_shader_src - The path to the fragment shader file
    return - The loaded shader
    */
    fn load_shader_from_file_with_geometry(&self, v_shader_src:&str, f_shader_src:&str, g_shader_src:&str) -> Shader {
        let vertex_code;
        let fragment_code;
        let geometry_code:String;

        vertex_code = self.read_shader_file(v_shader_src);
        fragment_code = self.read_shader_file(f_shader_src);
        geometry_code = self.read_shader_file(g_shader_src);
    
        let mut shader:Shader = Shader::new();
        shader.compile_with_geometry(&vertex_code, &fragment_code, &geometry_code);
        shader
    }

    /*
    Gets the text from the given shader file
    file - The path to the file containing the shader code
    return - The text of the shader file
    */
    fn read_shader_file(&self, file:&str) -> String {
        let code = fs::read_to_string(file).expect("Unable to load shader");
        code
    }

    /*
    Gets the texture of the passed in name from the ResourceManager if it exists
    name - The name of the texture used as a dictionary key
    return - The texture of that name if it exists in the dictionary
    */
    pub fn get_texture(&self, name:&str) -> &Texture {
        if self.textures.contains_key(name) {
            return self.textures.get(name).unwrap();
        }

        panic!("That texture {} doesn't exist", &name);
    }


    /*
    Loads a texture using the given file and inserts it into the textures dictionary using the name as the key
    src - The path to the texture file
    name - The name of the texture to be used as its key in the textures dictionary
    return - The loaded texture
    */
    pub fn load_texture(&mut self, src: &str, name:&str)->Texture {
        let mut texture = Texture::new();
        texture.generate(src);
        self.textures.insert(name.to_string(), texture);
        texture
    }

    /*
    Gets the texture of the passed in name from the ResourceManager if it exists
    name - The name of the texture used as a dictionary key
    return - The texture of that name if it exists in the dictionary
    */
    pub fn get_material(&mut self, name:&str) -> Material {
        if self.materials.contains_key(name) {
            return self.materials.get(name).unwrap().clone();
        }

        panic!("That material {} doesn't exist", &name);
    }

    pub fn get_model(&mut self, name:&str) -> Model {
        if self.models.contains_key(name) {
            return self.models.get(name).unwrap().clone();
        }

        panic!("That model {} doesn't exist", &name);
    }


    /*
    Loads a texture using the given file and inserts it into the textures dictionary using the name as the key
    src - The path to the texture file
    name - The name of the texture to be used as its key in the textures dictionary
    return - The loaded texture
    */
    pub fn load_material(&mut self, spec_src:Texture, dif_src:Texture, shininess:f32, name:&str) {
        
        self.materials.insert(name.to_string(), Material::new(dif_src, spec_src, shininess));
    }

    /*
    Creates a cube map from the given faces
    srcs - A vector of strings containing the paths to the faces of the cube map
    name - The name of the cube map
    return - The texture id of the cube map
    */
    pub fn load_cube_map(&self, srcs:Vec<&str>, name:&str) -> u32 {
        let mut cube_id:u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut cube_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, cube_id);
        }
       
        let mut handles = Vec::new();
        for i in 0..srcs.len() {
            let src = srcs[i].clone().to_string();
            let new_handle = thread::spawn(move||{
                let img = image::open(&Path::new(&src)).expect("Texture failed to load");
                let format = gl::RGB;

                let data = img.to_rgb8().into_raw();
                unsafe {
                    gl::TexImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32, 0, format as i32, img.width() as i32,
                                img.height() as i32, 0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
                }
            });
            
            handles.push(new_handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        unsafe {
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
            
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        
        
        cube_id
    }

    /*
    Loads the given json file and turns it into a Level object with lighting, shaders and models
    path - The path to the json file containing the Level info
    return - A Level object containing all the loaded information
    */
    pub fn load_level(&mut self, path:&str) -> Level {

        let code = fs::read_to_string(path).expect("Unable to load JSON");
        let v:Value = serde_json::from_str(&code).unwrap();

        let mut faces = Vec::new();
        for face in v["skybox"].as_array().unwrap() {
            faces.push(face.as_str().unwrap());
        }
    
        let level_loader = {
            let cube_id = self.load_cube_map(faces, "Skybox");
            let the_box = Skybox::new(cube_id);
            let level = Level::new(the_box);
            level
        };
        
        let level_handle = thread::spawn(||level_loader);

        let shader_loader = {
            let mut shaders:HashMap<String, Shader> = HashMap::new();
            for val in v["shaders"].as_array().unwrap() {
                let shader_name =  val["name"].as_str().unwrap().clone();
                
    
                let vert = val["vertex"].as_str().unwrap().to_string();
                let frag = val["fragment"].as_str().unwrap().to_string();
                
                let has_geo = !val["geometry"].is_null();
                if has_geo {
                    let geo = val["geometry"].as_str().unwrap().to_string();
                    let shader = self.load_shader_with_geometry(&vert, &frag, &geo, shader_name);
                    shaders.insert(shader_name.to_string(), shader);
                    
                } else {
                    let shader = self.load_shader(&vert, &frag, shader_name);
                    shaders.insert(shader_name.to_string(), shader);
                }
            }

            shaders
        };
       
        let shader_handle = thread::spawn(||shader_loader);
        let mat_loader = {
            let mut materials:HashMap<String, Material> = HashMap::new();
            for val in v["materials"].as_array().unwrap() {
            
                let material_name = val["name"].as_str().unwrap().to_string();
                let shininess = val["shininess"].as_f64().unwrap() as f32;
                let dif_src = val["diffuse"].as_str().unwrap();

                let spec_src = val["specular"].as_str().unwrap();
                
                let diffuse = self.load_texture(dif_src, "diffuse");
                let specular = self.load_texture(spec_src, "specular");            
                let material:Material = Material::new(diffuse, specular, shininess);
                println!("Loaded {} Material", material_name);
                materials.insert(material_name, material);

            }

            materials
        };

        let mat_handle = thread::spawn(||mat_loader);
        
        let mut level = level_handle.join().unwrap();
        self.shaders = shader_handle.join().unwrap();
        self.materials = mat_handle.join().unwrap();
        println!("Loading models");
        for val in v["models"].as_array().unwrap() {
            let model_name = val["name"].as_str().unwrap();
            
            let shader_name = val["shader"].as_str().unwrap();
            let shader = self.shaders[shader_name];
            
            let material = self.materials[val["material"].as_str().unwrap()];
            
            
            let path = val["path"].as_str().unwrap();
            let is_trigger = val["trigger"].as_bool().unwrap();
            let model = Model::new(path, shader, material, is_trigger);
            
            if val["instances"].is_array() {
                let mut game_object3d:GameObject3D = GameObject3D::new(model.clone());
                let instances = val["instances"].as_array().unwrap();
                
                for inst in instances{
                    let position = self.parse_vec3(inst["position"].to_owned(), false); 
                    let rotation = self.parse_vec3(inst["rotation"].to_owned(), true);
                    let scale = self.parse_vec3(inst["scale"].to_owned(), false);
        
                    let transform  = Transform::new(position, rotation, scale);
                    game_object3d.add_instance(transform);
                }

                game_object3d.init();
                level.add_game_object(game_object3d);
            }
            
            println!("Model {} has been loaded", model_name);
            self.models.insert(model_name.to_string(), model);
            
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

    /*
    Parses a 3-dimensional vector from the given json
    val - The json to be parsed
    convert - Whether the values should be converted to radians or not
    return - The 3-dimensional vector created from the json
    */
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