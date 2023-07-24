use glam::{vec2, Vec2};

use crate::drawer::{RectBlueprint, TextBlueprint, ImageSource, Drawer};
use crate::font::Font;
use crate::rect::Rect;

use self::image_renderer::ImageRenderer;
use self::rect_renderer::RectRenderer;
use self::text_renderer::TextRenderer;

mod array_buffer;
mod texture;
mod shader;

mod image_renderer;
mod rect_renderer;
mod text_renderer;

pub struct OpenglDrawer {
    pub dpi: f32,
    pub rect: Rect,
    pub alpha: f32,
    pub viewport: Vec2,
    pub default_font: Font<'static>,
    rect_renderer: RectRenderer,
    text_renderer: TextRenderer,
    image_renderer: ImageRenderer,
}

impl OpenglDrawer {
    #[doc(hidden)]
    pub fn new(w: f32, h: f32, dpi: f32, default_font: Font<'static>) -> Self {
        Self {
            dpi,
            viewport: vec2(w, h),
            rect: Rect::new(0., 0., w, h),
            rect_renderer: RectRenderer::new().unwrap(),
            text_renderer: TextRenderer::new(w, h, dpi).unwrap(),
            image_renderer: ImageRenderer::new(w, h).unwrap(),
            default_font,
            alpha: 1.0,
        }
    }
}

impl Drawer for OpenglDrawer {
    #[doc(hidden)]
    fn resize(&mut self, w: f32, h: f32, dpi: f32) {
        self.viewport = vec2(w, h);
        self.rect.w = w;
        self.rect.h = h;
        self.dpi = dpi;
        self.text_renderer.window_width = w;
        self.text_renderer.window_height = h;
        self.image_renderer.set_size(w, h);
    }

    fn begin_frame(&mut self) {
        self.text_renderer.begin_frame();
    }

    fn end_frame(&mut self) {
        self.text_renderer.end_frame();
    }

    fn draw_rect(&mut self, spec: &RectBlueprint) {
        self.rect_renderer.draw(spec);
    }

    fn draw_text(&mut self, spec: &TextBlueprint) {
        self.text_renderer.draw(spec);
    }

    fn draw_image(&mut self, rect: &Rect, image: &ImageSource) {
        self.image_renderer.draw(rect, image);
    }
}
