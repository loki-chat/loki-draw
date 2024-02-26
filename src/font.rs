use rusttype::{PositionedGlyph, Scale};

/// Represents a font.
///
/// To obtain a `Font`, use the [`use_font_data`](crate::hooks::use_font_data) hook.
pub struct Font<'a>(rusttype::Font<'a>);

impl<'a> Font<'a> {
    pub fn from_data(ttf_data: &'a [u8]) -> Self {
        Self(rusttype::Font::try_from_bytes(ttf_data).unwrap())
    }

    pub fn get_v_advance(&self, scale: Scale) -> f32 {
        let v_metrics = self.0.v_metrics(scale);
        v_metrics.ascent - v_metrics.descent + v_metrics.line_gap
    }

    /// Get width in pixels of a string of rendered text.
    pub fn text_width(&self, text: &str, size: f32) -> f32 {
        let scale = Scale::uniform(size);

        text.chars()
            .map(|c| {
                let scaled_glyph = self.0.glyph(c).scaled(scale);
                scaled_glyph.h_metrics().advance_width
            })
            .sum()
    }

    pub fn create_glyphs(&self, s: &str, x: f32, y: f32, size: f32) -> Vec<PositionedGlyph<'a>> {
        let mut glyphs = Vec::new();
        let mut glyph_pos = rusttype::point(x, y);
        let scale = rusttype::Scale::uniform(size);
        
        for c in s.chars() {
            let scaled_glyph = self.0.glyph(c).scaled(scale);
            let advance_width = scaled_glyph.h_metrics().advance_width;

            glyphs.push(scaled_glyph.positioned(glyph_pos));
            glyph_pos.x += advance_width;
        }

        glyphs
    }

    pub fn baseline(&self, size: f32) -> f32 {
        let scale = Scale::uniform(size);
        let v_metrics = self.0.v_metrics(scale);
        size + v_metrics.descent
    }
}
