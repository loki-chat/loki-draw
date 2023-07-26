use rusttype::{point, Point, PositionedGlyph, Scale};

/// Turns a glyph into a comparable value.
#[derive(PartialEq)]
pub enum GlyphComparator {
    ByValue(f32),
    ByCount(u32),
    //ByFullData([[f32; 30]; 30]),
}

impl GlyphComparator {
    pub fn new_by_value() -> Self {
        Self::ByValue(0.0)
    }

    pub fn new_by_count() -> Self {
        Self::ByCount(0)
    }

    // pub fn new_by_data() -> Self {
    //     Self::ByFullData([[0.0; 30]; 30])
    // }

    pub fn consumer<'a>(&'a mut self) -> impl FnMut(u32, u32, f32) + 'a {
        move |x, y, v| match self {
            &mut Self::ByValue(ref mut value) => *value += v,
            &mut Self::ByCount(ref mut count) => *count += 1,
            // &mut Self::ByFullData(ref mut data) => data[x as usize][y as usize] = v,
        }
    }
}

/// Represents a font.
///
/// To obtain a `Font`, use the [`use_font_data`](crate::hooks::use_font_data) hook.
#[derive(Clone)]
pub struct Font<'a>(rusttype::Font<'a>, &'a [u8]);

impl<'a> Font<'a> {
    pub fn from_data(ttf_data: &'a [u8]) -> Self {
        Self(rusttype::Font::try_from_bytes(ttf_data).unwrap(), ttf_data)
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

    pub fn create_glyphs(&self, s: &str, x: f32, y: f32, size: f32) -> Vec<PositionedGlyph<'a>> {
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

    pub fn is_char_probably_equal(&self, other: &Font<'_>, c: char) -> bool {
        let mut self_comp = GlyphComparator::new_by_count();
        self.0
            .glyph(c)
            .scaled(Scale { x: 30.0, y: 30.0 })
            .positioned(point(0.0, 0.0))
            .draw(self_comp.consumer());
        let mut other_comp = GlyphComparator::new_by_count();
        other
            .0
            .glyph(c)
            .scaled(Scale { x: 30.0, y: 30.0 })
            .positioned(point(0.0, 0.0))
            .draw(other_comp.consumer());
        self_comp == other_comp
    }

    pub fn is_bold(&self) -> bool {
        false // TODO
    }

    pub fn is_italic(&self) -> bool {
        false // TODO
    }
}

impl<'a> PartialEq for Font<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}
