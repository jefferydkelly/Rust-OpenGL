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
    
    let mut last_frame =glfw.get_time();
    let mut delta_time:f32 = 0.0;


    let quad_vertices:[f32;24] = [
        -1.0, 1.0, 0.0, 1.0,
        -1.0, -1.0, 0.0, 0.0,
        1.0, -1.0, 1.0, 0.0,
        
        -1.0, 1.0, 0.0, 1.0,
        1.0, -1.0, 1.0, 0.0,
        1.0, 1.0, 1.0, 1.0
    ];

    let mut fbo:u32 = 0;
    let mut quad_vao:u32 = 0;
    let mut tex_color_buffer:u32 = 0;
    
    let mut depth_map_fbo:u32 = 0;
    let (shadow_width, shadow_height) = (1024, 1024);
    let mut depth_map:u32 = 0;
    
    
    let cube_map_faces:Vec<&str> = vec![
        "src/resources/textures/skybox/right.jpg",
        "src/resources/textures/skybox/left.jpg",
        "src/resources/textures/skybox/top.jpg",
        "src/resources/textures/skybox/bottom.jpg",
        "src/resources/textures/skybox/front.jpg",
        "src/resources/textures/skybox/back.jpg",
    ];

    let sky_shader = ResourceManager::get_instance().load_shader("src/resources/shaders/skybox.vs", "src/resources/shaders/skybox.fs", "skybox");

    let skybox = Skybox::new(cube_map_faces);
    unsafe {
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
        
        
        gl::GenTextures(1, &mut tex_color_buffer);
        gl::BindTexture(gl::TEXTURE_2D, tex_color_buffer);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, SCR_WIDTH as i32, SCR_HEIGHT as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex_color_buffer, 0);

        let mut rbo:u32 = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, SCR_WIDTH as i32, SCR_HEIGHT as i32);
        //gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        gl::GenVertexArrays(1, &mut quad_vao);
        let mut quad_vbo:u32 = 0;
        gl::GenBuffers(1, &mut quad_vbo);

        gl::BindVertexArray(quad_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, quad_vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<f32>() * quad_vertices.len()) as isize, &quad_vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW);
        
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<f32>() as GLsizei, ptr::null());
    
        gl::GenFramebuffers(1, &mut depth_map_fbo);
        
        
        gl::GenTextures(1, &mut depth_map);
        gl::BindTexture(gl::TEXTURE_2D, depth_map);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT as i32, shadow_width, shadow_height, 0, gl::DEPTH_COMPONENT, gl::FLOAT, ptr::null());

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    
        let border_color:[f32;4] = [1.0, 1.0, 1.0, 1.0];
        gl::TextureParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, &border_color[0]);



        gl::BindFramebuffer(gl::FRAMEBUFFER, depth_map_fbo);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, depth_map, 0);
        gl::DrawBuffer(gl::NONE);
        gl::ReadBuffer(gl::NONE);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
     
    }
    let model_shader = ResourceManager::get_instance().get_shader("model").to_owned();
    model_shader.set_int("shadowMap", 0);
    
    let screen_shader= ResourceManager::get_instance().load_shader("src/resources/shaders/buffer.vs", "src/resources/shaders/buffers.fs", "screen");
    screen_shader.use_program();
    screen_shader.set_int("screenTexture", 0);

    let depth_shader= ResourceManager::get_instance().load_shader("src/resources/shaders/shadow.vs", "src/resources/shaders/shadow.fs", "shadow");

    while !window.should_close() {
        let current_frame = glfw.get_time();
        delta_time = (current_frame - last_frame) as f32;
        last_frame = current_frame;

            
        the_game.process_events(&mut window, &events);
        the_game.update(delta_time);
        
        unsafe {
            
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            
            //Configure shaders and matrices
            let (near_plane, far_plane) = (1.0, 7.5);
            let light_pos = vec3(-2.0, 4.0, -1.0);
            let light_projection = glm::ortho(-10.0, 10.0, -10.0, 10.0, near_plane, far_plane);
            let light_view = glm::look_at(&light_pos, &vec3(0.0, 0.0, 0.0), &vec3(0.0, 1.0, 0.0));
            let light_space_matrix = light_projection * light_view;
            
            
            depth_shader.use_program();
            depth_shader.set_matrix4("lightSpaceMatrix", &light_space_matrix);
            
            gl::Viewport(0, 0, shadow_width, shadow_height);
            gl::BindFramebuffer(gl::FRAMEBUFFER, depth_map_fbo);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
            gl::CullFace(gl::FRONT);
            the_game.render(&depth_shader);
            gl::CullFace(gl::BACK);

            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::Viewport(0,0, SCR_WIDTH as i32, SCR_HEIGHT as i32);

            model_shader.use_program();
            model_shader.set_vector3f_glm("lightPos", light_pos);
            model_shader.set_matrix4("lightSpaceMatrix", &light_space_matrix);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, depth_map);
            
            the_game.render(&model_shader);

            sky_shader.use_program();
            let mut skyview = the_game.get_view_matrix();
            skyview = glm::mat3_to_mat4(&glm::mat4_to_mat3(&skyview));
            sky_shader.set_matrix4("view", &skyview);
            sky_shader.set_matrix4("projection", &the_game.get_projection_matrix());
            sky_shader.set_int("skybox", 0);
            skybox.render();
            
            
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            screen_shader.use_program();
            gl::BindVertexArray(quad_vao);
            
            
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex_color_buffer);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

    

    window.swap_buffers();
    glfw.poll_events();
    }
}

