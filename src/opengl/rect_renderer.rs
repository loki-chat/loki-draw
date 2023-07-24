use glam::{vec4, Mat4, Vec4};

use crate::drawer::RectBlueprint;
use crate::rect::Rect;

use super::array_buffer::ArrayBuffer;
use super::shader::{self, AttribLocation, ShaderCompileError, ShaderProgram, UniformLocation};

fn compute_viewport_matrix(size: (f32, f32)) -> Mat4 {
    Mat4::orthographic_rh(0.0, size.0, size.1, 0.0, -1.0, 1.0)
}

fn color_rgba_from_u32(col: u32, alpha: f32) -> Vec4 {
    vec4(
        ((col & 0xff0000) >> 16) as f32 / 255.0,
        ((col & 0x00ff00) >> 8) as f32 / 255.0,
        (col & 0x0000ff) as f32 / 255.0,
        alpha,
    )
}

struct NineSlice {
    horiz: [f32; 4],
    vert: [f32; 4],
}

impl NineSlice {
    pub fn new(horiz: [f32; 4], vert: [f32; 4]) -> Self {
        Self { horiz, vert }
    }

    pub fn rect(&self, col: usize, row: usize) -> Rect {
        Rect {
            x: self.horiz[col],
            w: self.horiz[col + 1] - self.horiz[col],
            y: self.vert[row],
            h: self.vert[row + 1] - self.vert[row],
        }
    }

    pub fn edge_rect(&self, edge: usize, width: f32) -> Rect {
        match edge {
            0 => self.rect(1, 0).edge(0, width),
            1 => self.rect(2, 1).edge(1, width),
            2 => self.rect(1, 2).edge(2, width),
            3 => self.rect(0, 1).edge(3, width),
            _ => panic!("unknown edge"),
        }
    }

    pub fn corner_rect(&self, corner: usize) -> Rect {
        match corner {
            0 => self.rect(0, 0).hvflip(),
            1 => self.rect(2, 0).vflip(),
            2 => self.rect(2, 2),
            3 => self.rect(0, 2).hflip(),
            _ => panic!("unknown corner"),
        }
    }
}



/// Render rectangles.
pub struct RectRenderer {
    buf: ArrayBuffer,

    round_program: ShaderProgram,
    round_vertex: AttribLocation,
    round_mvp: UniformLocation,
    round_pos: UniformLocation,
    round_col: UniformLocation,
    round_size: UniformLocation,
    round_inner_rad: UniformLocation,
    round_smoothness: UniformLocation,

    square_program: ShaderProgram,
    square_vertex: AttribLocation,
    square_mvp: UniformLocation,
    square_pos: UniformLocation,
    square_col: UniformLocation,
    square_size: UniformLocation,
}

const RECT_VERT: &str = include_str!("shaders/rect.vert");
const RECT_FRAG: &str = include_str!("shaders/rect.frag");
const SQUARE_FRAG: &str = include_str!("shaders/square.frag");

impl RectRenderer {
    /// Create a RectRenderer
    pub fn new() -> Result<Self, ShaderCompileError> {
        let round_program = unsafe { shader::compile(RECT_VERT, RECT_FRAG) }?;
        let square_program = unsafe { shader::compile(RECT_VERT, SQUARE_FRAG) }?;

        let mut buf = ArrayBuffer::new(2);
        buf.set_data(vec![1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0]);

        Ok(RectRenderer {
            buf,

            round_program,
            round_vertex: round_program.get_attrib_location("vertex").unwrap(),
            round_mvp: round_program.get_uniform_location("mvp").unwrap(),
            round_pos: round_program.get_uniform_location("pos").unwrap(),
            round_col: round_program.get_uniform_location("col").unwrap(),
            round_size: round_program.get_uniform_location("size").unwrap(),
            round_inner_rad: round_program.get_uniform_location("inner_rad").unwrap(),
            round_smoothness: round_program.get_uniform_location("smoothness").unwrap(),

            square_program,
            square_vertex: square_program.get_attrib_location("vertex").unwrap(),
            square_mvp: square_program.get_uniform_location("mvp").unwrap(),
            square_pos: square_program.get_uniform_location("pos").unwrap(),
            square_col: square_program.get_uniform_location("col").unwrap(),
            square_size: square_program.get_uniform_location("size").unwrap(),
        })
    }

    fn round(&self, spec: &RectBlueprint, r: Rect, col: u32, inner: f32) {
        let m = compute_viewport_matrix(spec.viewport_size);
        let c = color_rgba_from_u32(col, spec.alpha);
        self.round_program.use_program();
        self.buf.bind(self.round_vertex, 0, 2);

        let size = (r.w.abs() + r.h.abs()) / 2.0;
        let smoothness = 1.0 / size;
        let inner_rad = inner / size;

        unsafe {
            gl::Uniform1f(self.round_smoothness.0, smoothness);
            gl::Uniform1f(self.round_inner_rad.0, inner_rad);
            gl::Uniform2f(self.round_pos.0, r.x, r.y);
            gl::Uniform2f(self.round_size.0, r.w, r.h);
            gl::Uniform4fv(self.round_col.0, 1, c.as_ref().as_ptr());
            gl::UniformMatrix4fv(self.round_mvp.0, 1, gl::FALSE, m.as_ref().as_ptr());
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, self.buf.len() as i32);
        }
    }

    fn square(&self, spec: &RectBlueprint, r: Rect, col: u32) {
        let m = compute_viewport_matrix(spec.viewport_size);
        let c = color_rgba_from_u32(col, spec.alpha);
        self.square_program.use_program();
        self.buf.bind(self.square_vertex, 0, 2);

        unsafe {
            gl::Uniform2f(self.square_pos.0, r.x, r.y);
            gl::Uniform2f(self.square_size.0, r.w, r.h);
            gl::Uniform4fv(self.square_col.0, 1, c.as_ref().as_ptr());
            gl::UniformMatrix4fv(self.square_mvp.0, 1, gl::FALSE, m.as_ref().as_ptr());
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, self.buf.len() as i32);
        }
    }

    /// Draw a rect, specified by orientation and size.
    pub fn draw(&self, spec: &RectBlueprint) {
        let r = &spec.rect;
        let cr = spec.corner_radius;
        let bw = spec.border_width;
        let ir = spec.corner_radius - spec.border_width;
        let n = NineSlice::new(
            [r.x, r.x + cr, r.x + r.w - cr, r.x + r.w],
            [r.y, r.y + cr, r.y + r.h - cr, r.y + r.h],
        );

        // Main
        self.round(spec, n.rect(0, 0).hvflip(), spec.color, 0.0);
        self.square(spec, n.rect(1, 0), spec.color);
        self.round(spec, n.rect(2, 0).vflip(), spec.color, 0.0);
        self.square(spec, n.rect(0, 1), spec.color);
        self.square(spec, n.rect(1, 1), spec.color);
        self.square(spec, n.rect(2, 1), spec.color);
        self.round(spec, n.rect(0, 2).hflip(), spec.color, 0.0);
        self.square(spec, n.rect(1, 2), spec.color);
        self.round(spec, n.rect(2, 2), spec.color, 0.0);

        // Corner borders
        for i in 0..4 {
            if spec.borders[i] && spec.borders[(i + 3) % 4] {
                self.round(spec, n.corner_rect(i), spec.border_color, ir);
            }
        }

        // Edge borders
        for i in 0..4 {
            if spec.borders[i] {
                self.square(spec, n.edge_rect(i, bw), spec.border_color);
            }
        }
    }
}
