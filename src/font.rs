use deborrow::deborrow;
use std::mem::MaybeUninit;

use swash::{
    scale::{image::Image, Render, ScaleContext, Source, StrikeWith},
    shape::{cluster::GlyphCluster, ShapeContext},
    text::Script,
    zeno::{Format, Vector},
    FontRef, Variation,
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
pub struct Font<'a>(Option<Box<[u8]>>, FontRef<'a>);

impl<'a> Font<'a> {
    pub fn from_data(ttf_data: &'a [u8], index: u32) -> Self {
        Self(None, FontRef::from_index(ttf_data, index as usize).unwrap())
    }
    pub fn from_data_vec(ttf_data: Vec<u8>, index: u32) -> Self {
        unsafe {
            // we immediately replace the uninit data with init data.
            let mut me: Font<'a> = Self(Some(ttf_data.into()), MaybeUninit::uninit().assume_init());
            // deborrowing here is fine because the Box will necessarily outlive the FontRef
            me.1 = FontRef::from_index(deborrow(me.0.as_ref().unwrap()), index as usize).unwrap();
            me
        }
    }

    fn get_glyph_advance(&self, c: char, s: f32) -> (f32, f32) {
        let mut scale_context = ScaleContext::new();
        let mut scaler = scale_context.builder(self.1).size(s).build();
        let scaled_glyph = scaler
            .scale_outline(self.1.charmap().map(c))
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

    pub fn render(&self, s: &str, x: f32, y: f32, size: f32) -> Vec<Image> {
        // initialize rendering configuration
        let mut scale_context = ScaleContext::new();
        let mut scaler = scale_context.builder(self.1).size(size).build();
        let mut render = Render::new(&[
            Source::ColorOutline(0),
            Source::ColorBitmap(StrikeWith::BestFit),
            Source::Outline,
        ]);
        render.format(Format::Subpixel);
        let mut offset = Vector::new(x, y);
        // get actual renderer
        let mut shape_context = ShapeContext::new();
        let mut shaper = shape_context
            .builder(self.1)
            .script(Script::Latin)
            .size(size)
            .build();
        // initialize shaper
        shaper.add_str(s);
        // start rendering
        let mut images = Vec::new();
        shaper.shape_with(|c| {
            for glyph in c.glyphs {
                offset = offset + Vector::new(glyph.x, glyph.y);
                // draw the glyph
                let mut image = render
                    .render(&mut scaler, glyph.id)
                    .unwrap_or_else(|| render.render(&mut scaler, 0).unwrap());
                // add correct positioning data to image
                image.placement.left = offset.x as i32;
                image.placement.top = offset.y as i32;
                images.push(image);
                // TODO check if this is all correct
                offset = offset + Vector::new(glyph.advance, 0.0);
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

    pub fn title(&self) -> String {
        // self.1
        //     .variations()
        //     .next()
        //     .expect("font without instance?")
        //     .name(None)
        //     .expect("font should have a name")
        //     .to_string()
        "Roboto".to_owned()
    }

    pub fn has(&self, chr: char) -> bool {
        self.1.charmap().map(chr) != 0
    }
}

impl<'a> PartialEq for Font<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.1.data == other.1.data
    }
}
