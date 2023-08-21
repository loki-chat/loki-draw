use std::fs;

use font_kit::{
    error::SelectionError,
    family_name::FamilyName,
    handle::Handle,
    properties::{Properties, Weight},
    source::SystemSource,
};

use crate::font::Font;

// Re-export so users dont need to put font_kit into their deps
pub type Style = font_kit::properties::Style;

pub trait StyleIsSlanted {
    fn is_slanted(&self) -> bool;
}

impl StyleIsSlanted for Style {
    fn is_slanted(&self) -> bool {
        match self {
            Style::Normal => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FontFamily {
    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
}

impl From<FontFamily> for FamilyName {
    fn from(value: FontFamily) -> Self {
        match value {
            FontFamily::Serif => FamilyName::Serif,
            FontFamily::SansSerif => FamilyName::SansSerif,
            FontFamily::Monospace => FamilyName::Monospace,
            FontFamily::Cursive => FamilyName::Cursive,
            FontFamily::Fantasy => FamilyName::Fantasy,
        }
    }
}

#[derive(Clone)]
pub struct TextSegment<'s, 'font>
where
    'font: 's,
{
    content: &'s str,
    index: usize,
    font: Font<'font>,
    pub size: f32,
    force_italicize: bool, // italicise by forceful image manipulation (for fonts that dont have italics)
    force_bold: bool, // bolden by forceful image manipulation (for fonts that dont have bold stuff)
}

impl<'s, 'font: 's> TextSegment<'s, 'font> {
    pub fn from_string(
        s: &'s str,
        scale: f32,
        italics: Style,
        bold: bool,
        italic_emoji: bool,
        bold_emoji: bool,
        font_family: FontFamily,
        preferred_font: Option<String>,
    ) -> Vec<Self> {
        let mut segments = Vec::new();
        let mut chars = s.chars();
        let mut i = 0;
        let mut last_font: Font<'font> = get_font_for(
            'A',
            italics,
            bold,
            font_family,
            preferred_font.clone(),
            None,
        );
        while let Some(c) = chars.next() {
            let font = get_font_for(
                c,
                italics,
                bold,
                font_family,
                preferred_font.clone(),
                Some(last_font),
            );
            last_font = font.clone();
            let mut force_italicize = !font.is_italic() && italics.is_slanted(); // if font cant do italics
            let mut force_bold = !font.is_bold() && bold; // if font cant do bold

            // is an emoji-like thing
            if is_nonmodifiable(c) {
                force_italicize = italics.is_slanted() && italic_emoji; // and if its an emoji, italicise it regardless of font because fonts dont italicize emojis
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
            i += 1;
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

const ROBOTO_FONT: &[u8] = include_bytes!("Roboto-Regular.ttf");

fn get_font_for<'font>(
    chr: char,
    italics: Style,
    bold: bool,
    font_family: FontFamily,
    preferred_font: Option<String>,
    last_font: Option<Font<'font>>,
) -> Font<'font> {
    let mut family = Vec::new();
    if let Some(last_font) = last_font {
        family.push(FamilyName::Title(last_font.title().to_owned()));
    }
    if let Some(preferred_font) = preferred_font {
        family.push(FamilyName::Title(preferred_font));
    }
    family.push(font_family.into());
    let font_source = SystemSource::new();
    // First, try getting the best matching font to the preferred one.
    let font = match font_source.select_best_match(
        &family,
        &Properties::new()
            .style(italics)
            .weight(font_kit::properties::Weight(if bold {
                700.0
            } else {
                400.0
            })),
    ) {
        Ok(x) => x,
        Err(SelectionError::NotFound) => {
            println!("No font found matching query: ");
            dbg!(chr, italics, bold, font_family);
            println!(
                "Falling back to builtin font. Text may look very off or have missing characters."
            );
            return Font::from_data(ROBOTO_FONT, 0);
        }
        Err(SelectionError::CannotAccessSource) => {
            println!("Couldn't access system fonts. Falling back to builtin font. Text may look very off or have missing characters.");
            return Font::from_data(ROBOTO_FONT, 0);
        }
    };
    // Now check if it has the character.

    let (path, offset) = match font {
        Handle::Path { path, font_index } => (
            path.into_os_string()
                .into_string()
                .expect("font paths is not unicode (reasonable assumption)"),
            font_index,
        ),
        Handle::Memory { .. } => {
            panic!("This font shouldn't be in memory??")
        }
    };

    let font = Font::from_data_vec(
        match fs::read(path) {
            Ok(x) => x,
            Err(_) => {
                println!("Couldn't access system fonts. Falling back to builtin font. Text may look very off or have missing characters.");
                return Font::from_data(ROBOTO_FONT, 0);
            }
        },
        offset,
    );

    if font.has(chr) {
        return font;
    }

    todo!()
}

fn is_nonmodifiable(c: char) -> bool {
    false
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

    pub fn compute<'font: 'fonts>(
        &mut self,
        scale: f32,
        italics: Style,
        italic_emoji: bool,
        bold: bool,
        bold_emoji: bool,
        font_family: FontFamily,
        preferred_font: Option<Font<'font>>,
    ) {
        let segments = TextSegment::from_string(
            self.string_text,
            scale,
            italics,
            bold,
            italic_emoji,
            bold_emoji,
            font_family,
            None,
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

    pub fn computed<'font: 'fonts>(
        mut self,
        scale: f32,
        italics: Style,
        italic_emoji: bool,
        bold: bool,
        bold_emoji: bool,
        font_family: FontFamily,
        preferred_font: Option<Font<'font>>,
    ) -> Self {
        self.compute(
            scale,
            italics,
            italic_emoji,
            bold,
            bold_emoji,
            font_family,
            preferred_font,
        );
        self
    }

    pub fn get_segments(&self) -> &Option<Vec<TextSegment<'s, 'fonts>>> {
        &self.segments
    }
}
