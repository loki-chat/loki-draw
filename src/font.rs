use rusttype::{point, Point, PositionedGlyph, Scale};

/// Represents a font.
///
/// To obtain a `Font`, use the [`use_font_data`](crate::hooks::use_font_data) hook.
pub struct Font<'a>(rusttype::Font<'a>);

impl<'a> Font<'a> {
    pub fn from_data(ttf_data: &'a [u8]) -> Self {
        Self(rusttype::Font::try_from_bytes(ttf_data).unwrap())
    }

    fn get_glyph_advance(&self, c: char, s: Scale) -> (f32, f32) {
        let g = self.0.glyph(c).scaled(s);
        let h = g.h_metrics().advance_width;
        let v_metrics = self.0.v_metrics(s);
        let v = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        (h, v)
    }

    /// Get width in pixels of a string of rendered text.
    pub fn get_str_width(&self, str: &str, size: f32) -> f32 {
        let mut w: f32 = 0.0;
        let s = Scale::uniform(size);

        for c in str.chars() {
            let (adv_x, _adv_y) = self.get_glyph_advance(c, s);
            w += adv_x;
        }

        w
    }

    pub fn create_glyphs(
        &self,
        s: &str,
        x: f32,
        y: f32,
        size: f32,
    ) -> Vec<PositionedGlyph<'a>> {
        let mut glyphs = Vec::new();
        let mut glyph_pos: Point<f32> = rusttype::point(x, y);
        let scale: Scale = rusttype::Scale::uniform(size);

        for c in s.chars() {
            let base_glyph = self.0.glyph(c);
            glyphs.push(base_glyph.scaled(scale).positioned(glyph_pos));

            let (adv_x, _adv_y) = self.get_glyph_advance(c, scale);
            glyph_pos = point(glyph_pos.x + adv_x, glyph_pos.y);
        }

        glyphs
    }

    pub fn baseline(&self, size: f32) -> f32 {
        let s = Scale::uniform(size);
        let v_metrics = self.0.v_metrics(s);
        size + v_metrics.descent
    }
}
