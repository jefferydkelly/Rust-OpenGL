pub enum GameState {
    ACTIVE,
    MENU,
    WIN,
    LOSE,
    TRANSITION
}

extern crate glfw;
use self::glfw::{Key, Action};

extern crate gl;
use self::gl::types::*;

use core::f32;
use std::{sync::mpsc::Receiver, mem, ptr};
use crate::engine::camera::Camera;
use crate::engine::game_object::GameObject;
use crate::engine::game_object3d::GameObject3D;
use crate::level::Level;
use crate::engine::lights::{DirectionalLight, PointLight, Spotlight};
use crate::engine::physics_manager::PhysicsManager;
use crate::engine::text_renderer::TextRenderer;
use crate::enemy::Enemy;
use crate::engine::textbox::TextBox;
use crate::engine::transform::Transform;
use crate::engine::ui_manager::UIManager;
use crate::player::Player;
use crate::engine::{input_manager::InputManager, resource_manager::ResourceManager, sprite_renderer::SpriteRenderer, traits::{Rendered, Updated}, model::Model};
use glm::{Vec3, vec2, vec3, Mat4};
use image::math;

use crate::engine::shader::Shader;

use super::model::Material;
use super::resource_manager;


pub struct Game {
    state:GameState,
    width: u32,
    height:u32,
    cammie:Camera,
    first_mouse:bool,
    level:Level,
    projection:Mat4
}

impl Game {
    /*
    Creates a new game object
    w - The width of the screen at the start of the game
    h - The height of the screen at the start of the game
    return - The game object
    */
    pub fn new(w:u32, h:u32)->Game {
   
        let mut levy = ResourceManager::get_instance().load_level("src/resources/json/test.json");
        let projection: Mat4 = glm::perspective(4.0 / 3.0, 45.0, 0.1, 500.0);

        for shader in ResourceManager::get_instance().get_all_shaders() {
            shader.use_program();
            shader.set_matrix4("projection", &projection);
        }
       
        let the_game = Game {
            state: GameState::ACTIVE,
            width: w,
            height: h,
            cammie: Camera::new(glm::vec3(0.0, 0.0, -50.0), glm::vec3(0.0, 1.0, 0.0), 90.0, 0.0),
            first_mouse: true,
            level: levy,
            projection: projection
        };

        the_game
    }


    /*
    Updates everything connected to the Game that needs it.
    dt - The time in seconds since the last update.
    */
    pub fn update(&mut self, dt:f32) {

        self.cammie.process_keyboard_input(dt);

        //self.player.update(dt);
        //self.the_box.update();
    }

    /*
    Renders the game using the given shader
    shader - The Shader to use when rendering the scene.
    */
    pub fn render(&mut self, shader:&Shader) {
        
        self.level.update_lighting(shader);
        shader.set_matrix4("view", &self.cammie.get_view_matrix());
        shader.set_vector3f_glm("viewPos", self.cammie.position);
        shader.set_vector3f_glm("spotlight.position", self.cammie.position);
        shader.set_vector3f_glm("spotlight.direction", self.cammie.forward);
    
        self.level.draw(shader);
        //self.player.render(&self.renderer);
        //self.ui.render();
    }

    /*
    Processes the events passed in
    window - A reference to the currently active window
    events - A list of events that have occurred
    */
    pub fn process_events(&mut self, window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
        for (_, event) in glfw::flush_messages(events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(key, _, action, _) => self.process_keyboard_events(window, key, action),
                glfw::WindowEvent::CursorPos(x, y) => self.process_mouse_events(x, y),
                _ => {}
            }
        }
    }
    
    /*
    Processes keyboard based events
    window - A reference to the currently active window
    key - The key the action was performed on
    action - The action performed on the key
    */
    fn process_keyboard_events(&mut self, window: &mut glfw::Window, key:glfw::Key, action: glfw::Action) {
        if key == Key::Escape && action == Action::Press {
            window.set_should_close(true)
        } else {
            unsafe {
                if action == Action::Press {
                    InputManager::instance().update_key_state(key, true);
                } else if action == Action::Release {
                    InputManager::instance().update_key_state(key, false);
                }
            }
        }
    }

    /*
    Processes mouse mvement and scrolling as they occur
    xpos - The current horizontal position of the mouse
    ypos - The current vertical position of the mouse
    */
    fn process_mouse_events(&mut self, xpos:f64, ypos:f64) {
        
        let mouse_pos = vec2(xpos as f32, ypos as f32);

        if self.first_mouse {
            InputManager::instance().update_mouse_position_glm(mouse_pos);
            self.first_mouse = false;
        }
        let mut mouse_dif = mouse_pos -  InputManager::instance().mouse_position;
        mouse_dif.y *= -1.0;
        self.cammie.process_mouse_movement(mouse_dif);
        InputManager::instance().update_mouse_position_glm(mouse_pos);
    }

    /*
    Returns the view matrix for outside use
    return - The camera's view matrix
    */
    pub fn get_view_matrix(&self) -> Mat4 {
        return self.cammie.get_view_matrix();
    }

    /*
    Returns the projection matrix for outside use
    return - The projection matrix
    */
    pub fn get_projection_matrix(&self) -> Mat4 {
        return self.projection;
    }
}