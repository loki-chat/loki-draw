use gl::types::*;

use glam::{vec4, Mat4, Vec2};
use rusttype::gpu_cache::Cache;
use rusttype::PositionedGlyph;

use crate::drawer::TextBlueprint;

use super::array_buffer::ArrayBuffer;
use super::shader::{self, AttribLocation, ShaderCompileError, ShaderProgram, UniformLocation};

/// Render text on screen.
pub struct TextRenderer {
    program: ShaderProgram,
    buf: ArrayBuffer,
    tex_id: u32,
    loc_vertex: AttribLocation,
    loc_tex_coord: AttribLocation,
    loc_col: UniformLocation,
    loc_mvp: UniformLocation,
    cache: Cache<'static>,
    used_glyphs: Vec<PositionedGlyph<'static>>,
}

const TEXT_VERT: &str = include_str!("shaders/text.vert");
const TEXT_FRAG: &str = include_str!("shaders/text.frag");

impl TextRenderer {
    /// Create a text renderer for a specified window size.
    pub fn new(dpi: f32) -> Result<Self, ShaderCompileError> {
        let cache: Cache<'static> = Cache::builder()
            .dimensions(0, 0)
            .scale_tolerance(dpi)
            .position_tolerance(dpi)
            .build();

        let mut tex_id: GLuint = 0;
        unsafe { gl::GenTextures(1, &mut tex_id) };

        let program = unsafe { shader::compile(TEXT_VERT, TEXT_FRAG) }?;

        let mut slf = Self {
            loc_vertex: program.get_attrib_location("vertex").unwrap(),
            loc_tex_coord: program.get_attrib_location("tex_coord").unwrap(),
            loc_col: program.get_uniform_location("col").unwrap(),
            loc_mvp: program.get_uniform_location("mvp").unwrap(),
            buf: ArrayBuffer::new(4),
            program,
            tex_id,
            cache,
            used_glyphs: vec![],
        };

        slf.set_cache_size(1);
        Ok(slf)
    }

    fn set_cache_size(&mut self, size: u32) {
        self.cache
            .to_builder()
            .dimensions(size, size)
            .rebuild(&mut self.cache);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::TexImage2D(
                gl::TEXTURE_2D, // target
                0,              // level
                gl::R8 as i32,
                size as i32, // width
                size as i32, // height
                0,           // border, must be 0
                gl::RED,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
        }
    }

    fn render_cache(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
        }

        let mut cache_misses = 0;
        let mut do_build = true;
        while do_build {
            let res = self.cache.cache_queued(|rect, data| {
                unsafe {
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
                    gl::TexSubImage2D(
                        gl::TEXTURE_2D,
                        0,
                        rect.min.x as i32,
                        rect.min.y as i32,
                        (rect.width()) as i32,
                        rect.height() as i32,
                        gl::RED,
                        gl::UNSIGNED_BYTE,
                        data.as_ptr() as *const _,
                    );
                }

                cache_misses += 1;
            });

            match res {
                Err(_) => {
                    cache_misses = 0;
                    self.set_cache_size(self.cache.dimensions().0 * 2);
                }
                Ok(_) => {
                    do_build = false;
                }
            };
        }
    }

    fn append_vertices(&self, glyph: &PositionedGlyph, pr: f32, data: &mut Vec<f32>) {
        let rect = self.cache.rect_for(0, glyph).unwrap();
        if rect.is_none() {
            return;
        }

        let (uv, screen) = rect.unwrap();
        data.extend([
            screen.min.x as f32 / pr,
            screen.min.y as f32 / pr,
            uv.min.x,
            uv.min.y,
            screen.max.x as f32 / pr,
            screen.min.y as f32 / pr,
            uv.max.x,
            uv.min.y,
            screen.max.x as f32 / pr,
            screen.max.y as f32 / pr,
            uv.max.x,
            uv.max.y,
            screen.min.x as f32 / pr,
            screen.min.y as f32 / pr,
            uv.min.x,
            uv.min.y,
            screen.max.x as f32 / pr,
            screen.max.y as f32 / pr,
            uv.max.x,
            uv.max.y,
            screen.min.x as f32 / pr,
            screen.max.y as f32 / pr,
            uv.min.x,
            uv.max.y,
        ]);
    }

    /// Draw text.
    pub fn draw(&mut self, viewport: Vec2, dpi: f32, spec: &TextBlueprint) {
        let m = Mat4::orthographic_rh(0.0, viewport.x, viewport.y, 0.0, -1.0, 1.0);
        let c = vec4(
            ((spec.col & 0xff0000) >> 16) as f32 / 255.0,
            ((spec.col & 0x00ff00) >> 8) as f32 / 255.0,
            (spec.col & 0x0000ff) as f32 / 255.0,
            spec.alpha,
        );

        let x = spec.x * dpi;
        let y = spec.y + spec.font.baseline(spec.size) * dpi;
        let glyphs = spec.font.create_glyphs(spec.text, x, y, spec.size * dpi);

        for glyph in &glyphs {
            self.used_glyphs.push(glyph.clone());
            self.cache.queue_glyph(0, glyph.clone());
        }

        self.render_cache();
        let mut data: Vec<f32> = vec![];
        for glyph in glyphs {
            self.append_vertices(&glyph, dpi, &mut data);
        }

        self.buf.set_data(data);

        self.program.use_program();
        self.buf.bind(self.loc_vertex, 0, 2);
        self.buf.bind(self.loc_tex_coord, 2, 2);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            gl::Uniform4fv(self.loc_col.0, 1, c.as_ref().as_ptr());
            gl::UniformMatrix4fv(self.loc_mvp.0, 1, gl::FALSE, m.as_ref().as_ptr());
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DrawArrays(gl::TRIANGLES, 0, self.buf.len() as i32);
        }
    }

    pub fn begin_frame(&mut self) {
        self.used_glyphs.clear();
    }

    pub fn end_frame(&mut self) {
        for glyph in self.used_glyphs.clone() {
            self.cache.queue_glyph(0, glyph);
        }

        self.render_cache();
        self.used_glyphs.clear();
    }
}
