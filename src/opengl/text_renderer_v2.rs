use crate::drawer::TextBlueprint;
use gl::types::{GLint, GLuint};
use glam::{vec4, Mat4, Vec2, Vec4};
use swash::scale::image::Content;

use crate::drawer::ImageSource;
use crate::rect::Rect;

use super::array_buffer::ArrayBuffer;
use super::shader::{self, AttribLocation, ShaderCompileError, ShaderProgram, UniformLocation};

pub struct TextRenderer {
    dpi: f32,
    program: ShaderProgram,
    buf: ArrayBuffer,
    loc_vertex: AttribLocation,
    loc_tex_coord: AttribLocation,
    loc_mvp: UniformLocation,
    loc_pos: UniformLocation,
    loc_size: UniformLocation,
    loc_col: UniformLocation,
}

const TEXT_VERT: &str = include_str!("shaders/text.vert");
const TEXT_FRAG: &str = include_str!("shaders/text.frag");

impl TextRenderer {
    pub fn new(dpi: f32) -> Result<Self, ShaderCompileError> {
        let program = unsafe { shader::compile(TEXT_VERT, TEXT_FRAG) }?;

        let mut buf = ArrayBuffer::new(4);
        buf.set_data(vec![
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0, // comments to keep alignment
            1.0, 0.0, 1.0, 1.0, 1.0, 1.0, // comments to keep alignment
            0.0, 0.0, 0.0, 0.0, 1.0, 1.0, // comments to keep alignment
            1.0, 1.0, 0.0, 1.0, 0.0, 1.0, // comments to keep alignment
        ]);

        Ok(Self {
            dpi,
            program,
            buf,
            loc_vertex: program.get_attrib_location("vertex").unwrap(),
            loc_tex_coord: program.get_attrib_location("tex_coord").unwrap(),
            loc_mvp: program.get_uniform_location("mvp").unwrap(),
            loc_pos: program.get_uniform_location("pos").unwrap(),
            loc_size: program.get_uniform_location("size").unwrap(),
            loc_col: program.get_uniform_location("col").unwrap(),
        })
    }

    pub fn draw(&self, viewport: glam::Vec2, spec: &TextBlueprint<'_>) {
        let matrix = Mat4::orthographic_rh(0.0, viewport.x, viewport.y, 0.0, -1.0, 1.0);
        let c = vec4(
            ((spec.col & 0xff0000) >> 16) as f32 / 255.0,
            ((spec.col & 0x00ff00) >> 8) as f32 / 255.0,
            (spec.col & 0x0000ff) as f32 / 255.0,
            spec.alpha,
        );

        // maybe join image together in future?
        // image.bind();

        self.program.use_program();
        self.buf.bind(self.loc_vertex, 0, 2);
        self.buf.bind(self.loc_tex_coord, 2, 2);

        unsafe {
            // TODO maybe move to draw_image_internal?
            gl::ActiveTexture(gl::TEXTURE0);

            gl::Uniform4fv(self.loc_col.0, 1, c.as_ref().as_ptr());
            gl::UniformMatrix4fv(self.loc_mvp.0, 1, gl::FALSE, matrix.as_ref().as_ptr());
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let segments = spec.text.get_segments().as_ref().unwrap();
        let mut x = 0.0;
        for segment in segments {
            println!("Rendering segment at x={x}.");
            let font = segment.get_font();
            for image in font.render(segment.get_text(), x, spec.y + segment.size, segment.size) {
                self.draw_image_internal(
                    image.placement.left as f32,
                    image.placement.top as f32,
                    image.placement.width,
                    image.placement.height,
                    &image.data,
                    matches!(image.content, Content::Mask),
                );
            }
            x += font.get_str_width(segment.get_text(), segment.size);
        }
    }

    fn draw_image_internal(
        &self,
        x: f32,
        y: f32,
        w: u32,
        h: u32,
        data: &[u8],
        is_alpha_only: bool,
    ) {
        println!("Rendering glyph: x={x} y={y} w={w} h={h} alpha={is_alpha_only}");
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32, // TODO find out what the diff between format and internalformat is, then allow custom formats
                w as i32,
                h as i32,
                0,
                if is_alpha_only { gl::ALPHA } else { gl::RGBA }, // TODO is ALPHA correct?
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            gl::Uniform2f(self.loc_pos.0, x, y - h as f32);
            gl::Uniform2f(self.loc_size.0, w as f32, h as f32);
            gl::DrawArrays(gl::TRIANGLES, 0, self.buf.len() as i32);
        }
    }
}
