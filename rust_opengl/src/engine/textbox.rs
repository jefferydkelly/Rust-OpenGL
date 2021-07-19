use std::str;

use crate::glm::{Vec2, Vec3, vec3};
use crate::engine::text_renderer::TextRenderer;
use crate::engine::traits::UI;
pub struct TextBox {
    position:Vec2,
    pub text:String,
    is_visible:bool,
    text_color:Vec3
}

impl TextBox {
    pub fn new(pos:Vec2, st:&str) -> Self {
        Self {
            position:pos,
            text:st.to_string(),
            is_visible:true,
            text_color:vec3(1.0, 1.0, 1.0)
        }
    }

    pub fn set_color(&mut self, color:Vec3) {
        self.text_color = color;
    }
}

impl UI for TextBox {

    fn set_visibility(&mut self, visible:bool) {
        self.is_visible = visible;
    }
    fn is_visible(&self)->bool {
        self.is_visible
    }

    fn set_position(&mut self, pos:Vec2) {
        self.position = pos;
    }

    fn get_position(&self)->Vec2 {
        self.position
    }

    fn render(&self, renderer:&TextRenderer) {
        if self.is_visible {
            renderer.draw_text(&self.text, self.position.x, self.position.y, 1.0, self.text_color);
        }
    }
}