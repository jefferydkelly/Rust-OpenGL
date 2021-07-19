use crate::engine::sprite_renderer::SpriteRenderer;
use crate::engine::text_renderer::TextRenderer;
use glm::Vec2;

pub trait Updated {
    fn update(&mut self, dt:f32);
}

pub trait Rendered {
    fn render(&self, renderer:&SpriteRenderer);
}

pub trait UI {
    fn set_visibility(&mut self, visible:bool);
    fn is_visible(&self)->bool;

    fn set_position(&mut self, pos:Vec2);
    fn get_position(&self)->Vec2;

    fn render(&self, renderer:&TextRenderer);

}