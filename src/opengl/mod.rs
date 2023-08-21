use glam::{vec2, Vec2};

use crate::drawer::{Drawer, ImageSource, RectBlueprint, TextBlueprint};
use crate::rect::Rect;

use self::image_renderer::ImageRenderer;
use self::rect_renderer::RectRenderer;
use self::text_renderer::TextRenderer;

mod array_buffer;
mod shader;
mod texture;

mod image_renderer;
mod rect_renderer;
mod text_renderer;

pub struct OpenglDrawer {
    pub dpi: f32,
    pub rect: Rect,
    pub alpha: f32,
    pub viewport: Vec2,
    rect_renderer: RectRenderer,
    text_renderer: TextRenderer,
    image_renderer: ImageRenderer,
}

impl OpenglDrawer {
    pub fn new(width: u32, height: u32, dpi: f32) -> Self {
        let (width, height) = (width as f32, height as f32);

        Self {
            dpi,
            viewport: vec2(width, height),
            rect: Rect::new(0., 0., width, height),
            rect_renderer: RectRenderer::new().unwrap(),
            text_renderer: TextRenderer::new(dpi).unwrap(),
            image_renderer: ImageRenderer::new().unwrap(),
            alpha: 1.0,
        }
    }
}

impl Drawer for OpenglDrawer {
    #[doc(hidden)]
    fn resize(&mut self, viewport: Vec2, dpi: f32) {
        self.viewport = viewport;
        self.rect.w = viewport.x;
        self.rect.h = viewport.y;
        self.dpi = dpi;

        unsafe {
            gl::Viewport(0, 0, viewport.x as i32, viewport.y as i32);
        }
    }

    fn begin_frame(&mut self) {}

    fn end_frame(&mut self) {}

    fn clear(&mut self) {
        unsafe {
            gl::ClearColor(0., 0., 0., 0.);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn draw_rect(&mut self, spec: &RectBlueprint) {
        self.rect_renderer.draw(self.viewport, spec);
    }

    fn draw_text(&mut self, spec: &TextBlueprint) {
        self.text_renderer.draw(self.viewport, spec);
    }

    fn draw_image(&mut self, rect: &Rect, image: &ImageSource) {
        self.image_renderer.draw(self.viewport, rect, image);
    }
}
