use glm::{Vec3, vec3, Mat4};

use crate::engine::shader::Shader;

#[derive(Clone, Copy)]
pub struct Transform2D {
    pub translation:Vec3,
    rotation:f32,
    pub scale:Vec3,
    pub model_matrix:Mat4
}

impl Transform2D {
    pub fn new(trans:Vec3, rot:f32, size:Vec3) -> Self {
        let mut matty = Mat4::identity();
        matty = glm::translate(&matty, &trans);
        matty = glm::rotate(&matty, rot, &vec3(0.0, 1.0, 0.0));
        matty = glm::scale(&matty, &size);


        Self {
            translation:trans,
            rotation:rot,
            scale:size,
            model_matrix:matty
        }
    }

    fn update_matrix(&mut self) {
     
        let mut matty = Mat4::identity();
        matty = glm::translate(&matty, &self.translation);
      
        matty = glm::rotate(&matty, self.rotation, &vec3(0.0, 0.0, 1.0));
        
        matty = glm::scale(&matty, &self.scale);
        self.model_matrix = matty;
    }

    pub fn update_translation(&mut self, trans:Vec3) {
        self.translation = trans;
        self.update_matrix();
    }

    pub fn translate(&mut self, trans:Vec3) {
        self.translation += trans;
        self.update_matrix();
    }

    pub fn get_translation(&self) -> Vec3 {
        self.translation
    }

    pub fn update_rotation(&mut self, rot:f32) {
        self.rotation = rot;
        self.update_matrix();
    }

    pub fn rotate(&mut self, rot:f32) {
        self.rotation += rot;
        self.update_matrix();
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn update_scale(&mut self, size:Vec3) {
        self.scale = size;
        self.update_matrix();
    }

    pub fn get_scale(&self) -> Vec3 {
        self.scale
    }

    pub fn apply_to_shader(&self, shader:&Shader) {
        shader.set_matrix4("model", &self.model_matrix);
    }
}