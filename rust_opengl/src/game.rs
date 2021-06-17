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
use crate::camera::Camera;
use crate::game_object::GameObject;
use crate::lights::{DirectionalLight, PointLight, Spotlight};
use crate::text_renderer::TextRenderer;
use crate::enemy::Enemy;
use crate::textbox::TextBox;
use crate::ui_manager::UIManager;
use crate::game_object3d::GameObject3D;

use crate::{input_manager::InputManager, player::Player, resource_manager::ResourceManager, sprite_renderer::SpriteRenderer, traits::{Controlled, Rendered, Updated}, model::Model};
use glm::{Vec3, vec2, vec3, Mat4};


pub struct Game {
    state:GameState,
    width: u32,
    height:u32,
    i_manager:InputManager,
    r_manager:ResourceManager,
    ui:UIManager,
    models:Vec<Model>,
    shady: crate::shader::Shader,
    cammie:Camera,
    house_pos: Vec3,
    first_mouse:bool
}

impl Game {
    pub fn new(w:u32, h:u32)->Game {
        let mut res_man = ResourceManager::new();
        
        let text_shader = res_man.load_shader("src/resources/shaders/textVertex.glsl", "src/resources/shaders/textFragment.glsl", "text");
        let mut uim = UIManager::new(text_shader);
        let text= "#ScreenshotSaturday";
        let tex = TextBox::new(vec2(200.0, 300.0), text);
        uim.add_element(Box::new(tex));

        let (the_models, directional_light) = res_man.load_json("src/resources/json/test.json");
        
        
        let house_shader = res_man.load_shader("src/resources/shaders/model.vs", "src/resources/shaders/model.fs", "model");
        house_shader.use_program();
        house_shader.set_int("tex", 0);
        
        let fov:f32 = 45.0;
        let projection:Mat4 = glm::perspective(w as f32/ h as f32, fov.to_radians(),0.1, 250.0);
        house_shader.set_matrix4("projection", &projection);
        
        let point_light_positions = [
            vec3(0.7, 0.2, 2.0),
            vec3(2.3, -3.3, -4.0),
            vec3(-4.0, 2.0, -12.0),
            vec3(0.0, 0.0, -3.0)];

        let points_lights = [PointLight {
            position: point_light_positions[0],
            ambient: vec3(0.05, 0.05, 0.05),
            diffuse: vec3(0.8, 0.8, 0.8),
            specular: vec3(1.0, 1.0, 1.0),
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032
        }, 
        PointLight {
            position: point_light_positions[1],
            ambient: vec3(0.05, 0.05, 0.05),
            diffuse: vec3(0.8, 0.8, 0.8),
            specular: vec3(1.0, 1.0, 1.0),
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032
        },
        PointLight {
            position: point_light_positions[2],
            ambient: vec3(0.05, 0.05, 0.05),
            diffuse: vec3(0.8, 0.8, 0.8),
            specular: vec3(1.0, 1.0, 1.0),
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032
        },
        PointLight {
            position: point_light_positions[3],
            ambient: vec3(0.05, 0.05, 0.05),
            diffuse: vec3(0.8, 0.8, 0.8),
            specular: vec3(1.0, 1.0, 1.0),
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032
        },];
        
        house_shader.set_dir_light("dirLight", directional_light);

        for i in 0..points_lights.len() {
            house_shader.set_point_light(&format!("pointLights[{}]", i), points_lights[i]);
        }

        let spotlight = Spotlight {
            position: vec3(0.0, 0.0, 0.0),
            direction: vec3(0.0, 0.0, 0.0),
            ambient: vec3(1.0, 1.0, 1.0),
            diffuse: vec3(0.8, 0.8, 0.8),
            specular: vec3(1.0, 1.0, 1.0),
            cutoff: 17.5,
            outer_cutoff: 22.5,
            constant: 0.1,
            linear: 0.04,
            quadratic: 0.0032
        };

        house_shader.set_spotlight("spotlight", spotlight);

        house_shader.set_vector3f("material.specular", 0.25, 0.25, 0.25);
        house_shader.set_float("material.shininess", 1.0);

       
        println!("I have {} models", the_models.len());
        
        Game {
            state: GameState::ACTIVE,
            width: w,
            height: h,
            i_manager: InputManager::new(),
            r_manager: res_man,
            ui: uim,
            models:the_models,
            shady: house_shader,
            house_pos: glm::vec3(0.0, 0.0, 5.0),
            cammie: Camera::new(glm::vec3(0.0, 0.0, -50.0), glm::vec3(0.0, 1.0, 0.0), -90.0, 0.0),
            first_mouse: true
        }
    }

    pub fn update(&mut self, dt:f32) {

        self.cammie.process_keyboard_input(self.i_manager, dt);
    }

    pub fn render(&mut self) {
        self.shady.use_program();
        
        self.shady.set_matrix4("view", &self.cammie.get_view_matrix());
        self.shady.set_vector3f_glm("viewPos", self.cammie.position);
        self.shady.set_vector3f_glm("spotlight.position", self.cammie.position);
        self.shady.set_vector3f_glm("spotlight.direction", self.cammie.forward);

        /*
        let mut mod_mat =glm::Mat4::identity();
        mod_mat = glm::translate(&mod_mat, &self.house_pos);
        mod_mat = glm::scale(&mod_mat, &vec3(0.5, 0.5, 0.5));
        self.shady.set_matrix4("model", &mod_mat);
        */
        for i in 0..self.models.len() {
            self.models[i].draw(&mut self.shady);
        }

        
        //self.ui.render();
    }

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
    
    fn process_keyboard_events(&mut self, window: &mut glfw::Window, key:glfw::Key, action: glfw::Action) {
        if key == Key::Escape && action == Action::Press {
            window.set_should_close(true)
        } else {
            unsafe {
                if action == Action::Press {
                    self.i_manager.update_key_state(key, true);
                } else if action == Action::Release {
                    self.i_manager.update_key_state(key, false);
                }
            }
        }
    }

    fn process_mouse_events(&mut self, xpos:f64, ypos:f64) {
        
        let mouse_pos = vec2(xpos as f32, ypos as f32);

        if self.first_mouse {
            self.i_manager.mouse_position = mouse_pos;
            self.first_mouse = false;
        }
        let mut mouse_dif = mouse_pos - self.i_manager.mouse_position;
        mouse_dif.y *= -1.0;
        self.cammie.process_mouse_movement(mouse_dif);
        self.i_manager.mouse_position = mouse_pos;
    }
}