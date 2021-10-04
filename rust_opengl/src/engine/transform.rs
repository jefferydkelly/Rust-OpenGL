use glm::{Vec3, vec3, Mat4};

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub translation:Vec3,
    rotation:Vec3,
    pub scale:Vec3,
    pub model_matrix:Mat4,
    pub unrotated_matrix:Mat4
}

impl Transform {
    pub fn new(trans:Vec3, rot:Vec3, size:Vec3) -> Self {
        let mut matty = Mat4::identity();
        
        
        matty = glm::translate(&matty, &trans);
        matty = glm::rotate(&matty, rot.x, &vec3(1.0, 0.0, 0.0));
        matty = glm::rotate(&matty, rot.y, &vec3(0.0, 1.0, 0.0));
        matty = glm::rotate(&matty, rot.z, &vec3(0.0, 0.0, 1.0));
        matty = glm::scale(&matty, &size);
        

        let mut rotty = Mat4::identity();
        rotty = glm::scale(&rotty, &size);
        rotty = glm::translate(&rotty, &trans);

        Self {
            translation:trans,
            rotation:rot,
            scale:size,
            model_matrix:matty,
            unrotated_matrix:rotty
        }
    }

    fn update_matrix(&mut self) {
        let mut matty = Mat4::identity();

        
        matty = glm::translate(&matty, &self.translation);
        matty = glm::rotate(&matty, self.rotation.x, &vec3(1.0, 0.0, 0.0));
        matty = glm::rotate(&matty, self.rotation.y, &vec3(0.0, 1.0, 0.0));
        matty = glm::rotate(&matty, self.rotation.z, &vec3(0.0, 0.0, 1.0));
        matty = glm::scale(&matty, &self.scale);
        
        self.model_matrix = matty;
        
        let mut rotty = Mat4::identity();
        rotty = glm::scale(&rotty, &self.scale);
        rotty = glm::translate(&rotty, &self.translation);
        
        self.unrotated_matrix = rotty;
    }

    pub fn update_translation(&mut self, trans:Vec3) {
        self.translation = trans;
        self.update_matrix();
    }

    pub fn translate(&mut self, trans:Vec3) {
        self.translation += trans;
        self.update_matrix();
    }

    pub fn update_rotation(&mut self, rot:Vec3) {
        self.rotation = rot;
        self.update_matrix();
    }

    pub fn rotate(&mut self, rot:Vec3) {
        self.rotation += rot;
        self.update_matrix();
    }

    pub fn reset_rotation(&mut self) {
        self.rotation = Vec3::zeros();
        self.update_matrix();
    }

    pub fn update_scale(&mut self, size:Vec3) {
        self.scale = size;
        self.update_matrix();
    }
}