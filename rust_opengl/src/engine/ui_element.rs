use std::ptr::null;

use crate::engine::text_renderer::TextRenderer;
use glm::Vec2;
use glm::Vec3;
use glm::vec3;
use nalgebra_glm::vec2;

use super::sprite_renderer::SpriteRenderer;
use super::texture::Texture;
use super::transform2d;
use super::transform2d::Transform2D;
use super::ui_renderer::UIRenderer;
#[derive(Debug)]
pub struct UIElement {
    transform:Transform2D,
    is_visible:bool,
    
    has_text:bool,
    text_color:Vec3,
    pub text:Option<String>,
    
    has_image:bool,
    sprite:Option<Texture>
}

impl UIElement {
    pub fn set_visibility(&mut self, visible:bool) {
        self.is_visible = visible;
    }
    pub fn is_visible(&self)->bool {
        self.is_visible
    }
    
    pub fn set_position(&mut self, pos:Vec2) {
        self.transform.update_translation(vec3(pos.x, pos.y, 0.0));
    }
    
    pub fn get_position(&self)->Vec3 {
        return self.transform.translation;
    }
    
    pub fn render_text(&self, renderer:&TextRenderer) {
        
        if self.is_visible && self.has_text{
            let the_text = self.text.as_ref().unwrap();
            let position = self.get_position();
            renderer.draw_text(&the_text, position.x, position.y, 1.0, self.text_color);
        }
    }

    pub fn render_image(&self, renderer:&UIRenderer) {
        if self.is_visible && self.has_image {
            renderer.draw_sprite(self.sprite.unwrap(), self.transform, vec3(1.0, 1.0, 1.0));
        }
    }

    pub fn new(pos:Vec2, st:&str) -> Self {
        let trans = Transform2D::new(vec3(pos.x, pos.y, 0.0), 0.0, vec3(100.0, 100.0, 0.0));
        Self {
            transform:trans,
            text:Some(st.to_owned()),
            is_visible:true,
            text_color:vec3(1.0, 1.0, 1.0),
            has_text:true,
            has_image:false,
            sprite: None
        }
    }

    pub fn new_image(pos:Vec2, spr:Texture) -> Self {
        let trans = Transform2D::new(vec3(pos.x, pos.y, 0.0), 0.0, vec3(100.0, 100.0, 0.0));
        Self {
            transform:trans,
            text:None,
            is_visible:true,
            text_color:vec3(1.0, 1.0, 1.0),
            has_text:false,
            has_image:true,
            sprite: Some(spr.to_owned())
        }
    }

    pub fn set_color(&mut self, color:Vec3) {
        self.text_color = color;
    }
}
