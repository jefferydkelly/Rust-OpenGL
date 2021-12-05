use std::ptr;

use glm::{Mat4, Vec2, Vec3, cross, look_at, vec3};
use na::ComplexField;
use crate::engine::input_manager::InputManager;

use super::transform::Transform;

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
    screen_size:Vec2,
    follow_target:*const Transform,
    follow_distance:f32,
    follow_vector:Vec3
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
            screen_size:screen_size,
            follow_target:ptr::null(),
            follow_distance: 15.0,
            follow_vector: vec3(0.0, 1.0, -2.0).normalize()
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

        if self.follow_target.is_null() {
            self.view = look_at(&self.position, &(self.position + &self.forward), &self.up);
        } else {
            let rotation = InputManager::get_instance().get_movement_input().x;
            if rotation.abs() > f32::EPSILON {
                self.follow_vector = glm::rotate_vec3(&self.follow_vector, rotation.to_radians() * 0.16, &self.up);
            }
            unsafe {
                let target_pos = (*self.follow_target).translation;
                self.view = look_at(&self.position, &target_pos, &self.up);
            }
        }
    }

    /*
        Provides the view matrix
        Return - The view matrix
    */
    pub fn get_view_matrix(&self) -> Mat4 {
        self.view
    }

    /*
        Provides the projection matrix
        Return - The projection matrix
    */
    pub fn get_projection_matrix(&self)->Mat4 {
        self.projection
    }

    /*
        Sets a target transform for the camera to follow
        target - A pointer to the transform to be followed 
    */
    pub fn set_follow_target(&mut self, target: *const Transform) {
        self.follow_target = target;
    }
    /* 
        Processes keyboard input and turns it into camera movement.
        dt - The time in seconds since the last update
    */
    pub fn update(&mut self, dt:f32) {
        
         if self.follow_target.is_null() {
            
            let mut movement = vec3(0.0, 0.0, 0.0);
            if  InputManager::get_instance().get_key_state(glfw::Key::Up) {
                movement += self.forward;
            } 

            if  InputManager::get_instance().get_key_state(glfw::Key::Down) {
                movement -= self.forward;
            } 

            if  InputManager::get_instance().get_key_state(glfw::Key::Left) {
                movement -= self.right;
            } 

            if  InputManager::get_instance().get_key_state(glfw::Key::Right) {
                movement += self.right;
            }
            self.move_camera(movement, dt);
        } else {
            unsafe {
                self.position = (*self.follow_target).translation + self.follow_vector * self.follow_distance;
                self.update_camera_vectors();
            }
        }

        
    }

    /*
        Moves the camera along the given vector according to speed and time
        offset - The unit vector to guide the camera's movement
        dt - The time (in seconds) since the last update
     */
    fn move_camera(&mut self, offset:Vec3, dt:f32) {
        let vel = self.move_speed * dt;
        self.position += offset * vel;
        self.update_camera_vectors();
    }
    /*
        Adjusts the rotation of the camera based on mouse movement
        offset - The movement of the mouse since the last calling of this function.
    */
    pub fn process_mouse_movement(&mut self, offset:Vec2) {
    
        if !InputManager::get_instance().is_gamepad() {
            let true_offset = offset.scale(self.mouse_sensitivity);
            self.yaw += true_offset.x;
            self.update_camera_vectors();
        }
    
    }

    /*
        Handles zooming betweeen the min and max amount
        offset - The change in zomom since the last frame.  
        - Controlled by mouse scroll on mouse and triggers on gamepad.
     */
    pub fn process_zoom(&mut self, offset:f32) {
        self.zoom -= offset;

        if self.zoom < 1.0 {
            self.zoom = 1.0;
        } else if self.zoom > 45.0 {
            self.zoom = 45.0;
        }

        self.projection = glm::perspective(self.screen_size.x / self.screen_size.y, self.zoom.to_radians(), 0.1, 500.0);

    }

    /*
        Handles alterations to the size of the game screen
        size - The new size of the screen 
    */
    pub fn update_screen_size(&mut self, size:Vec2) {
        self.screen_size = size;
        self.projection = glm::perspective(self.screen_size.x / self.screen_size.y, self.zoom, 0.1, 500.0);
    }
}