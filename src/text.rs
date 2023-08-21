use crate::font::Font;

#[derive(Clone)]
pub struct TextSegment<'s, 'font>
{
    content: &'s str,
    index: usize,
    font: Font<'font>,
    pub size: f32,
    force_italicize: bool, // italicise by forceful image manipulation (for fonts that dont have italics)
    force_bold: bool, // bolden by forceful image manipulation (for fonts that dont have bold stuff)
}

impl<'s, 'font> TextSegment<'s, 'font> {
    pub fn from_string(
        s: &'s str,
        scale: f32,
        italics: bool,
        bold: bool,
        italic_emoji: bool,
        bold_emoji: bool,
    ) -> Vec<Self> {
        let mut segments = Vec::new();
        for (i, c) in s.chars().enumerate() {
            let font = get_font_for(c, italics, bold);
            let mut force_italicize = !font.is_italic() && italics; // if font cant do italics
            let mut force_bold = !font.is_bold() && bold; // if font cant do bold

            // is an emoji-like thing
            if is_nonmodifiable(c) {
                force_italicize = italics && italic_emoji; // and if its an emoji, italicise it regardless of font because fonts dont italicize emojis
                force_bold = bold && bold_emoji; // same for bold
            }
            segments.push(Self {
                content: &s[i..=i],
                index: i,
                force_italicize,
                force_bold,
                font,
                size: scale,
            });
        }
        segments
    }

    pub fn can_combine_with(&self, other: &Self) -> bool {
        self.font == other.font
            && self.size == other.size
            && self.force_italicize == other.force_italicize
            && self.force_bold == other.force_bold
            && self.index + self.content.len() == other.index
    }

    pub fn combine_with(&mut self, other: &Self, origin: &'s str) {
        if self.index + self.content.len() != other.index {
            panic!("Tried to combine invalid text segments.");
        }
        self.content = &origin[self.index..other.index + other.content.len()];
    }

    pub fn get_font(&self) -> &Font {
        &self.font
    }

    pub fn get_text(&self) -> &str {
        self.content
    }

    pub fn should_force_bold(&self) -> bool {
        self.force_bold
    }

    pub fn should_force_italic(&self) -> bool {
        self.force_italicize
    }
}

fn get_font_for<'font>(_chr: char, _italics: bool, _bold: bool) -> Font<'font> {
    const ROBOTO_FONT: &[u8] = include_bytes!("../examples/common/Roboto-Regular.ttf");
    Font::from_data(ROBOTO_FONT)
}

fn is_nonmodifiable(_c: char) -> bool {
    false
}

pub struct Text<'s, 'fonts> {
    string_text: &'s str,
    segments: Option<Vec<TextSegment<'s, 'fonts>>>, // None if uncomputed
}

impl<'s, 'fonts> Text<'s, 'fonts> {
    pub fn new(string: &'s str) -> Self {
        Self {
            string_text: string,
            segments: None,
        }
    }

    pub fn set_text<'new_s: 's>(&mut self, new_text: &'new_s str) {
        self.string_text = new_text;
        self.segments = None;
    }

    pub fn compute(
        &mut self,
        scale: f32,
        italics: bool,
        italic_emoji: bool,
        bold: bool,
        bold_emoji: bool,
    ) {
        let segments = TextSegment::from_string(
            self.string_text,
            scale,
            italics,
            bold,
            italic_emoji,
            bold_emoji,
        );
        if segments.is_empty() {
            self.segments = Some(segments);
            return;
        }
        let mut current_segment = segments[0].clone();
        let mut new_segments = Vec::new();
        for segment in &segments[1..] {
            if current_segment.can_combine_with(segment) {
                current_segment.combine_with(segment, self.string_text);
            } else {
                new_segments.push(current_segment);
                current_segment = segment.clone();
            }
        }
        new_segments.push(current_segment);
        self.segments = Some(new_segments);
    }

    pub fn computed(
        mut self,
        scale: f32,
        italics: bool,
        italic_emoji: bool,
        bold: bool,
        bold_emoji: bool,
    ) -> Self {
        self.compute(scale, italics, italic_emoji, bold, bold_emoji);
        self
    }

    pub fn get_segments(&self) -> &Option<Vec<TextSegment<'s, 'fonts>>> {
        &self.segments
    }
}
