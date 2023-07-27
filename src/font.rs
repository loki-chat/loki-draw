use swash::{
    scale::{image::Image, Render, ScaleContext, Source, StrikeWith},
    shape::{cluster::GlyphCluster, ShapeContext},
    text::Script,
    zeno::{Format, Vector},
    FontRef,
};

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
pub struct Font<'a>(FontRef<'a>, &'a [u8]);

impl<'a> Font<'a> {
    pub fn from_data(ttf_data: &'a [u8]) -> Self {
        Self(FontRef::from_index(ttf_data, 0).unwrap(), ttf_data)
    }

    fn get_glyph_advance(&self, c: char, s: f32) -> (f32, f32) {
        let mut scale_context = ScaleContext::new();
        let mut scaler = scale_context.builder(self.0).size(s).build();
        let scaled_glyph = scaler
            .scale_outline(self.0.charmap().map(c))
            .unwrap_or_else(|| scaler.scale_outline(0).unwrap());
        let w = scaled_glyph.bounds().width();
        let v = scaled_glyph.bounds().height();
        (w, v)
    }

    /// Get width in pixels of a string of rendered text.
    pub fn get_str_width(&self, str: &str, size: f32) -> f32 {
        let mut w: f32 = 0.0;

        for c in str.chars() {
            let (adv_x, _adv_y) = self.get_glyph_advance(c, size);
            w += adv_x;
        }

        w
    }

    pub fn render(
        &self,
        s: &str,
        mut x: f32,
        mut y: f32,
        size: f32,
        color: [f32; 3],
    ) -> Vec<Image> {
        let mut scale_context = ScaleContext::new();
        let mut scaler = scale_context.builder(self.0).size(size).build();
        let mut render = Render::new(&[
            Source::ColorOutline(0),
            Source::ColorBitmap(StrikeWith::BestFit),
            Source::Outline,
        ]);
        render.format(Format::CustomSubpixel(color));
        let offset = Vector::new(x, y);
        let mut shape_context = ShapeContext::new();
        let mut shaper = shape_context
            .builder(self.0)
            .script(Script::Latin)
            .size(size)
            .build();
        shaper.add_str(s);
        let mut images = Vec::new();
        shaper.shape_with(|c| {
            for glyph in c.glyphs {
                let o = offset + Vector::new(glyph.x, glyph.y);
                images.push(
                    render
                        .offset(o)
                        .render(&mut scaler, glyph.id)
                        .unwrap_or_else(|| render.offset(o).render(&mut scaler, 0).unwrap()),
                );
            }
        });
        images
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
