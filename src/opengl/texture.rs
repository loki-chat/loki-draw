use gl::types::GLuint;

use crate::drawer::ImageSource;

impl ImageSource {
    pub fn from_memory(width: u32, height: u32, pixels_rgba: &[u8]) -> Self {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels_rgba.as_ptr() as *const _,
            );
        }

        Self { id, width, height }
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id) }
    }
}

impl Drop for ImageSource {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}
