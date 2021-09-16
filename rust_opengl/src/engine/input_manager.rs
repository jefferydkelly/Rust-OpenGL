
extern crate nalgebra_glm as glm;

use std::{iter::Once, ptr};

use glfw::{Glfw, ffi::{GLFWgamepadstate, glfwGetGamepadState, glfwJoystickIsGamepad, glfwJoystickPresent}};
use glm::{Vec2, vec2};
use na::ComplexField;
use once_cell::sync::OnceCell;



#[derive(Clone, Debug)]
pub struct InputManager {
    pub keys:[bool;1024],
    pub mouse_buttons:[bool;8],
    pub mouse_position:glm::Vec2,
    gamepad:glfw::Joystick
}

static mut INPUT_MANAGER:OnceCell<InputManager> = OnceCell::new();

impl InputManager {

    /*
    Creates the single instance of the Input Manager
    */
    pub fn create_instance(glf:Glfw) {
        let mut gamepad =  glfw::Joystick {
            id: glfw::JoystickId::Joystick1,
            glfw:glf
        };
        let many = InputManager {
            keys:[false;1024],
            mouse_buttons:[false;8],
            mouse_position:glm::vec2(0.0,0.0),
            gamepad:gamepad
        };
        unsafe {
            INPUT_MANAGER.set(many).unwrap();
        }
    }

    /*
    Returns the Input Manager singleton for use
    return - The Input Manager singleton
    */
    pub fn get_instance()->&'static mut InputManager {
        unsafe  {
            INPUT_MANAGER.get_mut().expect("Input Manager has not been created")
        }
    }

    /*
    Updates the key state of the given key to the given value
    key - The code for the key that was pressed
    is_down - Whether the key was pressed (true) or released (false)
    */
    pub fn update_key_state(&mut self, key:glfw::Key, is_down:bool) {
        let ukey = glfw::get_key_scancode(Some(key)).unwrap() as usize;
        if  ukey < 1024 {
            self.keys[ukey] = is_down;
        }

    }

    /*
    Gets the current state of the key
    key - The key to be checked
    return - The state of the key.  True if pressed.  False if not or the key doesn't exist/isn't tracked
    */
    pub fn get_key_state(&self, key:glfw::Key)->bool {
        let ukey = glfw::get_key_scancode(Some(key)).unwrap() as usize;
        if ukey < 1024 {
            return self.keys[ukey];
        }

        false
    }

    /*
    Updates the state of the mouse button
    button - The mouse button to be updated
    state - The current state of the button
    */
    pub fn update_mouse_buttons(&mut self, button:usize, state:f32) {
        if button < 8 {
            self.mouse_buttons[button] = state > 0.0;
        }
    }

    /*
    Gets the current state of the mouse button
    button - The button to be checked
    return - True if the mouse button is clicked or false if it is not or is out of range of the array
    */
    pub  fn get_mouse_button_state(&self, button:usize) -> bool {
        if button < 8 {
            return self.mouse_buttons[button];
        }

        false
    }

    /*
    Updates the stored position of the mouse
    x - The x position of the mouse
    y - The y position of the mouse
    */
    pub fn update_mouse_position(&mut self, x:f32, y:f32) {
        self.mouse_position = glm::vec2(x,y);
    }

    /*
    Updates the stored position of the mouse
    pos - A 2-dimensional vector representing the current position of the mouse
    */
    pub fn update_mouse_position_glm(&mut self, pos:Vec2) {
        self.mouse_position = pos;
    }

    /*
    Returns the current position of the mouse for outside use
    return - A 2-dimensional vector representing the current position of the mouse
    */
    pub fn get_mouse_position(&self)->glm::Vec2 {
        self.mouse_position
    }

    pub fn is_gamepad(&self) -> bool {
        return self.gamepad.is_gamepad();
    }

    pub fn get_gamepad_input(&self) -> Vec2 {
        if self.is_gamepad() {
            let state = self.gamepad.get_gamepad_state().unwrap();
            let mut x = state.get_axis(glfw::GamepadAxis::AxisLeftX);

            if x.abs() < 0.1 {
                x = 0.0;
            }
            let mut y = state.get_axis(glfw::GamepadAxis::AxisLeftY);

            if y.abs() < 0.1 {
                y = 0.0;
            }

            return vec2(x,-y);
        }

        return vec2(0.0, 0.0);
    }
    /*
    Resets the Input Manager by setting all stored values to false
    */
    pub fn clear_input(&mut self) {
        for i in 0..self.keys.len() {
            self.keys[i] = false;
        }

        for i in 0..self.mouse_buttons.len() {
            self.mouse_buttons[i] = false;
        }
    }
}