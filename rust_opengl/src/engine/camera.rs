use glm::{Mat4, Vec2, Vec3, cross, look_at, vec3};
use crate::engine::input_manager::InputManager;

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
    zoom: f32,
    view:Mat4,
    projection:Mat4,
    screen_size:Vec2
}

impl Camera {
    /*
    Creates a new Camera object with the given information
    pos - The position of the camera in world space
    upw - The world's up vector
    y - The starting yaw of the Camera
    pit - The starting pitch of the Camera
    */
    pub fn new(pos:Vec3, upw:Vec3, y:f32, pit:f32, screen_size:Vec2)-> Camera {
        let fwd = vec3(0.0, 0.0, 1.0);
        let fov = 45.0;
        let projection = glm::perspective(screen_size.x / screen_size.y, fov, 0.1, 500.0);
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
            view: look_at(&pos, &(pos + &fwd), &upw),
            projection:projection,
            screen_size:screen_size 
        };
        cammie.update_camera_vectors();
        cammie
    }

    /*
    Creates the forward vector based on the current rotation of the camera 
    and updates up and right vectors based on the result
    */
    fn update_camera_vectors(&mut self) {
        let x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        let y = self.pitch.to_radians().sin();
        let z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.forward = vec3(x,y,z).normalize();
        
        self.right = cross(&self.forward, &self.world_up).normalize();
        self.up = cross(&self.right, &self.forward).normalize();

        self.view = look_at(&self.position, &(self.position + &self.forward), &self.up);
    }

    /*
    Provides the view matrix
    Return - The view matrix
    */
    pub fn get_view_matrix(&self) -> Mat4 {
        self.view
    }

    pub fn get_projection_matrix(&self)->Mat4 {
        self.projection
    }

    /* 
    Processes keyboard input and turns it into camera movement.
    dt - The time in seconds since the last update
    */
    pub fn process_keyboard_input(&mut self, dt:f32) {
        let vel = self.move_speed * dt;
        if InputManager::get_instance().is_gamepad() {
            let input = InputManager::get_instance().get_gamepad_left_stick();
            self.position += vel * input.y * self.forward;
            self.position += vel * input.x * self.right;

            let offset = InputManager::get_instance().get_gamepad_right_stick();
        
            self.yaw += offset.x;
            self.pitch += offset.y;
            
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            } else if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            let scroll = InputManager::get_instance().get_gamepad_right_trigger() - InputManager::get_instance().get_gamepad_left_trigger();
            self.process_mouse_scroll(scroll / 10.0);
        } else {
            if  InputManager::get_instance().get_key_state(glfw::Key::W) {
                self.position += self.forward * vel;
            } 

            if  InputManager::get_instance().get_key_state(glfw::Key::S) {
                self.position -= self.forward * vel;
            } 

            if  InputManager::get_instance().get_key_state(glfw::Key::A) {
                self.position -= self.right* vel;
            } 

            if  InputManager::get_instance().get_key_state(glfw::Key::D) {
                self.position += self.right* vel;
            }
        }

        self.update_camera_vectors();
    }

    /*
    Adjusts the rotation of the camera based on mouse movement
    offset - The movement of the mouse since the last calling of this function.
    */
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

    pub fn process_mouse_scroll(&mut self, offset:f32) {
        self.zoom -= offset;

        if self.zoom < 1.0 {
            self.zoom = 1.0;
        } else if self.zoom > 45.0 {
            self.zoom = 45.0;
        }

        self.projection = glm::perspective(self.screen_size.x / self.screen_size.y, self.zoom.to_radians(), 0.1, 500.0);

    }
}