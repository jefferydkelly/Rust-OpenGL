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

use core::f32;
use std::{sync::mpsc::Receiver, mem, ptr, ffi::c_void};
use crate::engine::camera::Camera;

use crate::engine::resource_manager::ResourceManager;
use crate::level::Level;
use glm::{vec2, Mat4};
use nalgebra_glm::{Vec2, vec3};

use crate::engine::shader::Shader;

use super::aabb::AABB;
use super::physics::{self, Physics};
use super::input_manager::InputManager;
use super::ui_manager::UIManager;
use super::ui_element::UIElement;


pub struct Game {
    state:GameState,
    width: u32,
    height:u32,
    cammie:Camera,
    first_mouse:bool,
    level:Level,
    projection:Mat4,
    model_shader:Shader,
    screen_shader:Shader,
    depth_shader:Shader,
    skybox_shader:Shader,
    frame_buffer:u32,
    quad_vao:u32,
    uniform_buffer:u32,
    texture_color_buffer:u32,
    depth_map_buffer:u32,
    shadow_map:u32,
    shadow_size:Vec2,
    physics_manager:Physics
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
        
        UIManager::create_instance(w as f32,h as f32);

        let text = UIElement::new(vec2(600.0, 500.0), "TEST");
        UIManager::get_instance().add_element(text);
        
        let projection = glm::perspective(4.0 / 3.0, 45.0, 0.1, 500.0);
        let model_shader = ResourceManager::get_instance().get_shader("model").to_owned();
        model_shader.use_program();
        model_shader.set_int("shadowMap", 0);
        
        let screen_shader= ResourceManager::get_instance().load_shader("src/resources/shaders/buffer.vs", "src/resources/shaders/buffers.fs", "screen");
        screen_shader.use_program();
        screen_shader.set_int("screenTexture", 0);

        let depth_shader= ResourceManager::get_instance().load_shader("src/resources/shaders/shadow.vs", "src/resources/shaders/shadow.fs", "shadow");
        let sky_shader = ResourceManager::get_instance().load_shader("src/resources/shaders/skybox.vs", "src/resources/shaders/skybox.fs", "skybox");
        sky_shader.use_program();
        sky_shader.set_matrix4("projection", &projection);

        let abby = AABB::new(vec3(-10.0, -10.0, -10.0), vec3(10.0, 10.0, 10.0));
        let ybba = AABB::new(vec3(-10.0, -10.0, 20.0), vec3(10.0, 10.0,30.0));
        let mut phy_man = Physics::new();
        phy_man.add_body(abby);
        phy_man.add_body(ybba);
        let the_game = Game {
            state: GameState::ACTIVE,
            width: w,
            height: h,
            cammie: Camera::new(glm::vec3(0.0, 0.0, -50.0), glm::vec3(0.0, 1.0, 0.0), 90.0, 0.0),
            first_mouse: true,
            level: levy,
            projection: projection,
            model_shader: model_shader,
            screen_shader: screen_shader,
            depth_shader: depth_shader,
            skybox_shader: sky_shader,
            frame_buffer:0,
            quad_vao:0,
            uniform_buffer: 0,
            texture_color_buffer:0,
            depth_map_buffer:0,
            shadow_map:0,
            shadow_size: vec2(0.0, 0.0),
            physics_manager:phy_man
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

        let cam_pos = self.cammie.position;
        let cam_fwd = self.cammie.forward;
        self.level.update_lighting(&self.model_shader);
        self.model_shader.set_matrix4("view", &self.cammie.get_view_matrix());
        self.model_shader.set_vector3f_glm("viewPos", cam_pos);
        self.model_shader.set_vector3f_glm("spotlight.position", cam_pos);
        self.model_shader.set_vector3f_glm("spotlight.direction", cam_fwd);

        
    }

    /*
    Renders the game using the given shader
    shader - The Shader to use when rendering the scene.
    */
    pub fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            
            //Configure shaders and matrices
            let (near_plane, far_plane) = (1.0, 7.5);
            let light_pos = vec3(-2.0, 4.0, -1.0);
            let light_projection = glm::ortho(-10.0, 10.0, -10.0, 10.0, near_plane, far_plane);
            let light_view = glm::look_at(&light_pos, &vec3(0.0, 0.0, 0.0), &vec3(0.0, 1.0, 0.0));
            let light_space_matrix = light_projection * light_view;
            
            
            self.depth_shader.use_program();
            self.depth_shader.set_matrix4("lightSpaceMatrix", &light_space_matrix);
            
            gl::Viewport(0, 0, self.shadow_size.x as i32, self.shadow_size.y as i32);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.depth_map_buffer);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
            gl::CullFace(gl::FRONT);
            self.level.draw(&self.depth_shader);
            gl::CullFace(gl::BACK);

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.frame_buffer);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::Viewport(0,0, self.width as i32, self.height as i32);

            self.model_shader.use_program();
            self.model_shader.set_vector3f_glm("lightPos", light_pos);
         
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.shadow_map);
            
            let mat_size = mem::size_of::<Mat4>() as isize;
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.uniform_buffer);
      
            let view_mat = self.cammie.get_view_matrix(); 
            let view = view_mat.as_ptr();
            gl::BufferSubData(gl::UNIFORM_BUFFER, mat_size, mat_size, view as *const c_void);

            let light =  light_space_matrix.as_ptr();
            gl::BufferSubData(gl::UNIFORM_BUFFER, mat_size * 2, mat_size, light as *const c_void);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
            self.model_shader.set_uniform_block("Matrices", 0);
            
            
            gl::CullFace(gl::FRONT);
            self.level.draw(&self.model_shader);
            gl::CullFace(gl::BACK);

            self.skybox_shader.use_program();
            let skyview = glm::mat3_to_mat4(&glm::mat4_to_mat3(&self.cammie.get_view_matrix()));
            self.skybox_shader.set_matrix4("view", &skyview);
            self.skybox_shader.set_matrix4("projection", &self.projection);
            self.skybox_shader.set_int("skybox", 0);
            self.level.draw_skybox();
            
            UIManager::get_instance().render();
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.screen_shader.use_program();
            gl::BindVertexArray(self.quad_vao);
            
            
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_color_buffer);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            
        }
    
        
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
            
            if action == Action::Press {
                InputManager::get_instance().update_key_state(key, true);
            } else if action == Action::Release {
                InputManager::get_instance().update_key_state(key, false);
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
            InputManager::get_instance().update_mouse_position_glm(mouse_pos);
            self.first_mouse = false;
        }
        let mut mouse_dif = mouse_pos -  InputManager::get_instance().mouse_position;
        mouse_dif.y *= -1.0;
        self.cammie.process_mouse_movement(mouse_dif);
        InputManager::get_instance().update_mouse_position_glm(mouse_pos);
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

    pub fn resize_window(&mut self, size:Vec2) {
        self.width = size.x as u32;
        self.height = size.y as u32;
    }

    pub fn initialize_render_data(&mut self) {

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

        let mut ubo:u32 = 0;
        let mat_size = mem::size_of::<Mat4>() as isize;
        unsafe {
            
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            
            
            gl::GenTextures(1, &mut tex_color_buffer);
            gl::BindTexture(gl::TEXTURE_2D, tex_color_buffer);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, self.width as i32, self.height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex_color_buffer, 0);

            let mut rbo:u32 = 0;
            gl::GenRenderbuffers(1, &mut rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, self.width as i32, self.height as i32);
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
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<f32>() as i32, ptr::null());
        
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

            gl::GenBuffers(1, &mut ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER, ubo);
                        
            gl::BufferData(gl::UNIFORM_BUFFER, 3 * mat_size, ptr::null(), gl::STATIC_DRAW);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);

            gl::BindBufferRange(gl::UNIFORM_BUFFER, 0, ubo, 0, 3 * mat_size);

            gl::BindBuffer(gl::UNIFORM_BUFFER, ubo);
            let proj = self.projection.as_ptr();
            gl::BufferSubData(gl::UNIFORM_BUFFER, 0, mat_size, proj as *const c_void);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
            
            self.model_shader.use_program();
            self.model_shader.set_uniform_block("Matrices", 0);
        }

        self.frame_buffer = fbo;
        self.depth_map_buffer = depth_map_fbo;
        self.shadow_map = depth_map;
        self.texture_color_buffer = tex_color_buffer;
        self.quad_vao = quad_vao;
        self.uniform_buffer = ubo;
        self.shadow_size = vec2(shadow_width as f32, shadow_height as f32);
    }
}