use crate::traits::UI;
use crate::shader::Shader;
use crate::text_renderer::TextRenderer;
use crate::gl;
pub struct UIManager {
    elements:Vec<Box<dyn UI>>,
    renderer:TextRenderer
}

impl UIManager {
    
    pub fn new(text:Shader)->Self {
        let mut tex_rex = TextRenderer::new(800, 600, text);
        tex_rex.load_font("src/resources/fonts/arial.ttf", 48);
        Self {
            elements: Vec::new(),
            renderer:tex_rex
        }
    }

    pub fn render(&mut self) {
        unsafe {
            //gl::Disable(gl::DEPTH_TEST);
            let rend = &self.renderer;
            for e in self.elements.iter() {
                e.render(rend);
            }
            //gl::Enable(gl::DEPTH_TEST);
        }
    }

    pub fn add_element(&mut self, ui:Box<dyn UI>) {
        self.elements.push(ui);
    }
}