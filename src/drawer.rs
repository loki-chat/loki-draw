use crate::font::Font;
use crate::rect::Rect;

pub struct RectBlueprint {
    pub viewport_size: (f32, f32),
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
    #[doc(hidden)]
    fn resize(&mut self, w: f32, h: f32, dpi: f32);

    fn begin_frame(&mut self);
    fn end_frame(&mut self);

    fn draw_rect(&mut self, spec: &RectBlueprint);
    fn draw_text(&mut self, spec: &TextBlueprint);
    fn draw_image(&mut self, rect: &Rect, image: &ImageSource);
}