use crate::font::Font;

#[derive(Clone)]
pub struct TextSegment<'s, 'font>
where
    'font: 's,
{
    content: &'s str,
    index: usize,
    font: Font<'font>,
    scale: f32,
    force_italicize: bool, // italicise by forceful image manipulation (for fonts that dont have italics)
    force_bold: bool, // bolden by forceful image manipulation (for fonts that dont have bold stuff)
}

impl<'s, 'font: 's> TextSegment<'s, 'font> {
    pub fn from_string(
        s: &'s str,
        scale: f32,
        italics: bool,
        bold: bool,
        italic_emoji: bool,
        bold_emoji: bool,
    ) -> Vec<Self> {
        let mut segments = Vec::new();
        let mut chars = s.chars();
        let mut i = 0;
        while let Some(c) = chars.next() {
            let font = get_font_for(c, italics, bold);
            let mut force_italicize = !font.is_italic() && italics; // if font cant do italics
            let mut force_bold = !font.is_bold() && bold; // if font cant do bold
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
                scale,
            });
            i += 1;
        }
        segments
    }

    pub fn can_combine_with(&self, other: &Self) -> bool {
        self.font == other.font
            && self.scale == other.scale
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
}

fn get_font_for<'font>(chr: char, italics: bool, bold: bool) -> Font<'font> {}

fn is_nonmodifiable(c: char) -> bool {
    get_font_for(c, true, false).is_char_probably_equal(&get_font_for(c, false, false), c)
}

pub struct Text<'s, 'fonts: 's> {
    string_text: &'s str,
    segments: Option<Vec<TextSegment<'s, 'fonts>>>, // None if uncomputed
}

impl<'s, 'fonts: 's> Text<'s, 'fonts> {
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
        let mut segments = TextSegment::from_string(
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

    pub fn get_segments(&self) -> &Option<Vec<TextSegment<'s, 'fonts>>> {
        &self.segments
    }
}
