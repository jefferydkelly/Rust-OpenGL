use crate::engine::shader::Shader;
use crate::engine::text_renderer::TextRenderer;
use crate::engine::ui_element::UIElement;
use once_cell::sync::OnceCell;

use super::{resource_manager::ResourceManager, sprite_renderer::{self, SpriteRenderer}, ui_renderer::UIRenderer};

static mut UI_MANAGER:OnceCell<UIManager> = OnceCell::new();

#[derive(Debug)]
pub struct UIManager {
    elements:Vec<UIElement>,
    text_renderer:TextRenderer,
    sprite_renderer:UIRenderer
}

impl UIManager {
    

    pub fn new(width:f32, height:f32, text:Shader, ui:Shader) -> Self {
        text.set_vector3f("textColor", 1.0, 1.0, 1.0);
        let mut tex_rex = TextRenderer::new(800, 600, text);
        tex_rex.load_font("src/resources/fonts/arial.ttf", 48);

        //let mut rend = SpriteRenderer::new(sprite);
        let mut rend = UIRenderer::new(width, height, ui);
        rend.init_render_data();
        
        Self {
            elements:Vec::new(),
            text_renderer:tex_rex,
            sprite_renderer: rend
        }
    }
    /*
        Creates a new instance of the UI Manager
        width - The width of the screen
        height - The height of the screen
    */
    pub fn create_instance(width:f32, height:f32) {
        let text_shader = ResourceManager::get_instance().get_shader("model");//ResourceManager::get_instance().load_shader("src/resources/shaders/text.vs", "src/resources/shaders/text.fs", "text");

        let ui_sprite_shader = ResourceManager::get_instance().get_shader("model");//ResourceManager::get_instance().load_shader("src/resources/shaders/text.vs", "src/resources/shaders/basicShader.fs", "ui");
        text_shader.set_vector3f("textColor", 1.0, 1.0, 1.0);
        let mut tex_rex = TextRenderer::new(800, 600, text_shader);
        tex_rex.load_font("src/resources/fonts/arial.ttf", 48);

        //let mut rend = SpriteRenderer::new(sprite);
        let mut rend = UIRenderer::new(width, height, ui_sprite_shader);
        rend.init_render_data();
        
        let many = UIManager {
            elements:Vec::new(),
           text_renderer:tex_rex,
           sprite_renderer: rend
        };

        unsafe {
            UI_MANAGER.set(many).unwrap();
        }
    }

    /*
        Grants access to the current instance of UI Manager
        return - The UI Manager singleton
    */
    pub fn get_instance()->&'static mut UIManager {
        unsafe  {
            UI_MANAGER.get_mut().expect("UI Manager has not been created")
        }
    }


    /*
        Renders the UI 
    */
    pub fn render(&mut self) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            for e in self.elements.iter() {
                e.render_image(&self.sprite_renderer);
                e.render_text(&self.text_renderer);
            }
            gl::Disable(gl::BLEND);
        }
    }

    /*
        Adds a new UI element under the managers control
        ui - The new UI element 
    */
    pub fn add_element(&mut self, ui:UIElement) {
        self.elements.push(ui);
    }
}