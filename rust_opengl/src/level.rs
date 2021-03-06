
use crate::engine::{game_object3d::GameObject3D, lights::*, model::Model, resource_manager::ResourceManager, shader::Shader, skybox::Skybox};
use glm::Mat4;
pub struct Level {
    game_objects:Vec<GameObject3D>,
    dir_lights: Vec<DirectionalLight>,
    point_lights: Vec<PointLight>,
    spotlights: Vec<Spotlight>,
    skybox: Skybox
}

impl Level {
    pub fn new(the_box:Skybox) -> Self {
        Self {
            game_objects: Vec::new(),
            dir_lights: Vec::new(),
            point_lights: Vec::new(),
            spotlights: Vec::new(),
            skybox:the_box
        }
    }

    pub fn add_game_object(&mut self, game_object:GameObject3D) {
        self.game_objects.push(game_object);
    }

    pub fn add_directional_light(&mut self, light:DirectionalLight) {
        self.dir_lights.push(light);
    }

    pub fn add_point_light(&mut self, light:PointLight) {
        self.point_lights.push(light);
    }

    pub fn add_spotlight(&mut self, light:Spotlight) {
        self.spotlights.push(light);
    }

    pub fn add_lighting_to_shader(&self, shader:Shader) -> Shader {
        for i in 0..self.dir_lights.len() {
            shader.set_dir_light("dirLight", self.dir_lights[i]);
        }

        for i in 0..self.point_lights.len() {
            shader.set_point_light(&format!("pointLights[{}]", i), self.point_lights[i]);
        }

        shader.set_int("numPointLights", self.point_lights.len() as i32);

        for i in 0..self.spotlights.len() {
            shader.set_spotlight("spotlight", self.spotlights[i]);
        }

        shader
    }

    pub fn update_lighting(&mut self, shader:&Shader) {
        
        shader.use_program();
        for i in 0..self.dir_lights.len() {
            shader.set_dir_light("dirLight", self.dir_lights[i]);
        }

        for i in 0..self.point_lights.len() {
            shader.set_point_light(&format!("pointLights[{}]", i), self.point_lights[i]);
        }

        shader.set_int("numPointLights", self.point_lights.len() as i32);

        for i in 0..self.spotlights.len() {
            shader.set_spotlight("spotlight", self.spotlights[i]);
        }
        
    }

    pub fn draw(&mut self, shader:&Shader) {
        for i in 0..self.game_objects.len() {
            self.game_objects[i].draw(shader);
        }
    }

    pub fn draw_skybox(&self) {
        self.skybox.render();
    }
}
