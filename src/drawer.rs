use glam::Vec2;

use crate::font::Font;
use crate::rect::Rect;

pub struct RectBlueprint {
    pub rect: Rect,
    pub color: u32,
    pub border_color: u32,
    pub border_width: f32,
    pub corner_radius: f32,
    pub borders: [bool; 4],
    pub alpha: f32,
}

pub struct TextBlueprint<'a> {
    pub text: &'a str,
    pub x: f32,
    pub y: f32,
    pub font: &'a Font<'static>,
    pub size: f32,
    pub col: u32,
    pub alpha: f32,
}

impl<'a> TextBlueprint<'a> {
    pub fn text_width(&self) -> f32 {
        self.font.text_width(self.text, self.size)
    }

    pub fn text_height(&self) -> f32 {
        self.font.get_v_advance(rusttype::Scale::uniform(self.size))
    }
}

/// An image to be used with the [img](appy::components::Img) component.
///
/// An image source can only be created from memory, e.g. together with
/// the `include_bytes!` macro. There is currently no way to load an
/// image from a file or other assets.
#[derive(Debug)]
pub struct ImageSource {
    pub(crate) id: u32,
    pub width: u32,
    pub height: u32,
}

pub trait Drawer {
    fn resize(&mut self, viewport: Vec2, dpi: f32);

    fn begin_frame(&mut self);
    fn end_frame(&mut self);

    fn clear(&mut self);
    fn draw_rect(&mut self, spec: &RectBlueprint);
    fn draw_text(&mut self, spec: &TextBlueprint);
    fn draw_image(&mut self, rect: &Rect, image: &ImageSource);
}
