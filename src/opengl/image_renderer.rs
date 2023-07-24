use gl::types::GLint;
use glam::Mat4;

use crate::drawer::ImageSource;
use crate::rect::Rect;

use super::array_buffer::ArrayBuffer;
use super::shader::{self, AttribLocation, ShaderCompileError, ShaderProgram, UniformLocation};

pub struct ImageRenderer {
    program: ShaderProgram,
    buf: ArrayBuffer,
    window_width: f32,
    window_height: f32,
    loc_vertex: AttribLocation,
    loc_tex_coord: AttribLocation,
    loc_mvp: UniformLocation,
    loc_pos: UniformLocation,
    loc_size: UniformLocation,
}

const IMAGE_VERT: &str = include_str!("shaders/image.vert");
const IMAGE_FRAG: &str = include_str!("shaders/image.frag");

impl ImageRenderer {
    pub fn new(window_width: f32, window_height: f32) -> Result<Self, ShaderCompileError> {
        let program = unsafe { shader::compile(IMAGE_VERT, IMAGE_FRAG) }?;

        let mut buf = ArrayBuffer::new(4);
        buf.set_data(vec![
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        ]);

        Ok(Self {
            program,
            buf,
            window_width,
            window_height,
            loc_vertex: program.get_attrib_location("vertex").unwrap(),
            loc_tex_coord: program.get_attrib_location("tex_coord").unwrap(),
            loc_mvp: program.get_uniform_location("mvp").unwrap(),
            loc_pos: program.get_uniform_location("pos").unwrap(),
            loc_size: program.get_uniform_location("size").unwrap(),
        })
    }

    pub fn set_size(&mut self, window_width: f32, window_height: f32) {
        self.window_width = window_width;
        self.window_height = window_height;
    }

    pub fn draw(&self, rect: &Rect, image: &ImageSource) {
        let m = Mat4::orthographic_rh(0.0, self.window_width, self.window_height, 0.0, -1.0, 1.0);

        image.bind();

        self.program.use_program();
        self.buf.bind(self.loc_vertex, 0, 2);
        self.buf.bind(self.loc_tex_coord, 2, 2);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            gl::Uniform2f(self.loc_pos.0, rect.x, rect.y);
            gl::Uniform2f(self.loc_size.0, rect.w, rect.h);
            gl::UniformMatrix4fv(self.loc_mvp.0, 1, gl::FALSE, m.as_ref().as_ptr());
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DrawArrays(gl::TRIANGLES, 0, self.buf.len() as i32);
        }
    }
}
