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


pub struct Game {
    state:GameState,
    width: u32,
    height:u32,
    ui:UIManager,
    cammie:Camera,
    first_mouse:bool,
    renderer:SpriteRenderer,
    player:Player
}

impl Game {
    pub fn new(w:u32, h:u32)->Game {
   
        let text_shader = ResourceManager::get_instance().load_shader("src/resources/shaders/textVertex.glsl", "src/resources/shaders/textFragment.glsl", "text");
        let mut uim = UIManager::new(text_shader);
        let text= "#ScreenshotSaturday";
        let tex = TextBox::new(vec2(200.0, 300.0), text);
        uim.add_element(Box::new(tex));

        let sprite_shader = ResourceManager::get_instance().load_shader("src/resources/shaders/spriteVertex.glsl", "src/resources/shaders/spriteFragment.glsl", "sprite");
        let mut sprite_renderer = SpriteRenderer::new(&sprite_shader);
        sprite_renderer.init_render_data(800.0, 600.0);
        
        let mut my_sprites:Vec<Box<&Rendered>> = Vec::new();
        let player_tex = ResourceManager::get_instance().load_texture("src/resources/textures/playerShip.png", "player");
        let laser_tex = ResourceManager::get_instance().load_texture("src/resources/textures/laser.png", "laser");
        let player = Player::new(vec3(400.0, 300.0, 0.0), vec3(50.0, 50.0, 1.0), player_tex);

        let mut objs:Vec<Box<&Updated>> = Vec::new();
        objs.push(Box::new(&player));
        my_sprites.push(Box::new(&player));
        Game {
            state: GameState::ACTIVE,
            width: w,
            height: h,
            ui: uim,
            cammie: Camera::new(glm::vec3(0.0, 0.0, -50.0), glm::vec3(0.0, 1.0, 0.0), 90.0, 0.0),
            first_mouse: true,
            renderer:sprite_renderer,
            player:player
        }
    }

    pub fn update(&mut self, dt:f32) {

        //self.cammie.process_keyboard_input(dt);
        self.player.update(dt);
        //self.the_box.update();
    }

    pub fn render(&mut self) {
        
        self.player.render(&self.renderer);
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
                    InputManager::instance().update_key_state(key, true);
                } else if action == Action::Release {
                    InputManager::instance().update_key_state(key, false);
                }
            }
        }
    }

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
}