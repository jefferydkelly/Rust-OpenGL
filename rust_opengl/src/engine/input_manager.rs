
extern crate nalgebra_glm as glm;

use std::iter::Once;

use glm::Vec2;
use once_cell::sync::OnceCell;



#[derive(Clone, Copy, Debug)]
pub struct InputManager {
    pub keys:[bool;1024],
    pub mouse_buttons:[bool;8],
    pub mouse_position:glm::Vec2
}

static mut INPUT_MANAGER:OnceCell<InputManager> = OnceCell::new();

impl InputManager {
    pub fn create_instance() {
        let many = InputManager {
            keys:[false;1024],
            mouse_buttons:[false;8],
            mouse_position:glm::vec2(0.0,0.0)
        };
        unsafe {
            INPUT_MANAGER.set(many).unwrap();
        }
    }

    pub fn instance()->&'static mut InputManager {
        unsafe  {
            INPUT_MANAGER.get_mut().expect("Input Manager has not been created")
        }
    }

    pub fn update_key_state(&mut self, key:glfw::Key, is_down:bool) {
        let ukey = glfw::get_key_scancode(Some(key)).unwrap() as usize;
        if  ukey < 1024 {
            self.keys[ukey] = is_down;
        }

    }

    pub fn get_key_state(&self, key:glfw::Key)->bool {
        let ukey = glfw::get_key_scancode(Some(key)).unwrap() as usize;
        if ukey < 1024 {
            return self.keys[ukey];
        }

        false
    }

    pub fn update_mouse_buttons(&mut self, button:usize, state:f32) {
        if button < 8 {
            self.mouse_buttons[button] = state > 0.0;
        }
    }

    pub  fn get_mouse_button_state(&self, button:usize) -> bool {
        if button < 8 {
            return self.mouse_buttons[button];
        }

        false
    }

    pub fn update_mouse_position(&mut self, x:f32, y:f32) {
        self.mouse_position = glm::vec2(x,y);
    }

    pub fn update_mouse_position_glm(&mut self, pos:Vec2) {
        self.mouse_position = pos;
    }

    pub fn get_mouse_position(&self)->glm::Vec2 {
        self.mouse_position
    }

    pub fn clear_input(&mut self) {
        for i in 0..self.keys.len() {
            self.keys[i] = false;
        }

        for i in 0..self.mouse_buttons.len() {
            self.mouse_buttons[i] = false;
        }
    }
}