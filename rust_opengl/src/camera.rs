use glm::{Mat4, Vec2, Vec3, cross, look_at, vec3};
use crate::input_manager::InputManager;

pub struct Camera {
    pub position:Vec3,
    pub forward:Vec3,
    up:Vec3,
    right:Vec3,
    world_up:Vec3,
    yaw:f32,
    pitch:f32,
    move_speed:f32,
    mouse_sensitivity: f32,
    zoom: f32
}

impl Camera {
    pub fn new(pos:Vec3, upw:Vec3, y:f32, pit:f32)-> Camera {
        let mut cammie = Camera {
            position:pos,
            forward: vec3(0.0, 0.0, 1.0),
            up: vec3(0.0, 0.0, 0.0),
            right: vec3(0.0, 0.0, 0.0),
            world_up: upw,
            yaw: y,
            pitch: pit,
            move_speed: 25.0,
            mouse_sensitivity: 0.1,
            zoom: 45.0,

        };
        cammie.update_camera_vectors();
        cammie
    }

    fn update_camera_vectors(&mut self) {
        let x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        let y = self.pitch.to_radians().sin();
        let z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.forward = vec3(x,y,z).normalize();
        
        self.right = cross(&self.forward, &self.world_up).normalize();
        self.up = cross(&self.right, &self.forward).normalize();
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        return look_at(&self.position, &(self.position + self.forward), &self.up);
    }

    pub fn process_keyboard_input(&mut self, manager:InputManager, dt:f32) {
        let vel = self.move_speed * dt;

        if manager.get_key_state(glfw::Key::W) {
            self.position += self.forward * vel;
        } 

        if manager.get_key_state(glfw::Key::S) {
            self.position -= self.forward * vel;
        } 

        if manager.get_key_state(glfw::Key::A) {
            self.position -= self.right* vel;
        } 

        if manager.get_key_state(glfw::Key::D) {
            self.position += self.right* vel;
        }
    }

    //TODO: Process Mouse Movement and Scroll
    pub fn process_mouse_movement(&mut self, offset:Vec2) {
        let true_offset = offset.scale(self.mouse_sensitivity);
        
        self.yaw += true_offset.x;
        self.pitch += true_offset.y;
        
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        } else if self.pitch > 89.0 {
            self.pitch = 89.0;
        }

        self.update_camera_vectors();
    
    }
}