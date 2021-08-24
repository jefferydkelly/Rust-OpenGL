#![allow(non_upper_case_globals)]
extern crate glfw;
extern crate memoffset;
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
extern crate gl;

use std::{mem, ptr, ffi::c_void};

use engine::{audio_manager::AudioManager, resource_manager};
use engine::game::Game;
use engine::physics_manager::PhysicsManager;
use gl::types::GLsizei;
use glm::{Mat3, Mat4, mat3, mat4, vec3};
use crate::engine::skybox::Skybox;
use crate::engine::{input_manager::InputManager, resource_manager::ResourceManager};


use self::glfw::{Context};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;


pub mod player;
pub mod enemy;
pub mod level;
pub mod engine;

pub fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.set_cursor_pos_polling(true);
    
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::Enable(gl::DEPTH_TEST);
        //gl::DepthFunc(gl::LESS);
        //gl::Enable(gl::CULL_FACE);
        //gl::Enable(gl::BLEND | gl::DEPTH_TEST);
        //gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    
    InputManager::create_instance();
    ResourceManager::create_instance();
    PhysicsManager::create_instance();
    AudioManager::create_instance();
    
    let mut the_game = Game::new(SCR_WIDTH, SCR_HEIGHT);
    the_game.initialize_render_data();
    let mut last_frame =glfw.get_time();
    let mut delta_time:f32 = 0.0;

    while !window.should_close() {
        let current_frame = glfw.get_time();
        delta_time = (current_frame - last_frame) as f32;
        last_frame = current_frame;

            
        the_game.process_events(&mut window, &events);
        the_game.update(delta_time);
        the_game.render();
    

        window.swap_buffers();
        glfw.poll_events();
    }
}

