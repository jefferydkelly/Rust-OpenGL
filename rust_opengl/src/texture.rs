extern crate gl;
use std::{ffi::c_void, path::Path};

use image::{self, GenericImageView};

#[derive(Clone, Copy, Debug)]
pub struct Texture {
    pub id:u32,
    width:i32,
    height:i32,
    pub internal_format:u32,
    pub image_format:u32,
    wrap_s:u32,
    wrap_t:u32,
    filter_min:u32,
    filter_max:u32,
    pub name:&'static str
}

impl Texture {

    pub fn new()->Self {
        Self {
            width: 0,
            height: 0,
            internal_format: gl::RGB,
            image_format: gl::RGB,
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            filter_min: gl::LINEAR,
            filter_max: gl::LINEAR,
            id: 0,
            name: "texture"
        }
    }

    pub fn generate(&mut self, file:&str) {
        unsafe {
            gl::GenTextures(1, &mut self.id);

           
            let mut img = image::open(&Path::new(file)).expect("Texture failed to load");
            let format = gl::RGBA;

            let data = img.to_rgba8().into_raw();

            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, img.width() as i32,
                           img.height() as i32, 0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}