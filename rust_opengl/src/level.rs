
use crate::engine::{lights::*, model::Model, shader::Shader};
pub struct Level {
    models:Vec<Model>,
    dir_lights: Vec<DirectionalLight>,
    point_lights: Vec<PointLight>,
    spotlights: Vec<Spotlight>
}

impl Level {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            dir_lights: Vec::new(),
            point_lights: Vec::new(),
            spotlights: Vec::new()
        }
    }

    pub fn add_model(&mut self, milly:Model) {
        self.models.push(milly);
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

    pub fn draw(&self) {
        for i in 0..self.models.len() {
            self.models[i].draw();
        }
    }
}
