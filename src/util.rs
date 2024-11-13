use crate::resources::Resources;
use crate::style::{Class, Style};
use crate::Config;
use printpdf::Pt;
use rusttype::{Font, Scale};
use std::borrow::Cow;

pub fn width_of_text(resources: &Resources, style: &Style, text: &str) -> Pt {
    let font = font_from_style(resources, style);
    let scale = scale_from_style(resources.get_config(), style);
    let units_per_em = font.units_per_em() as f32;
    let glyph_space_width: f32 = font
        .glyphs_for(text.chars())
        .map(|g| g.clone().scaled(scale).h_metrics().advance_width)
        .sum();
    Pt(glyph_space_width * scale.x / units_per_em)
}

pub fn font_height(resources: &Resources, style: &Style) -> Pt {
    let font = font_from_style(resources, style);
    let scale = scale_from_style(resources.get_config(), style);
    let v_metrics = font.v_metrics(scale);
    let height = (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap) as f64;
    Pt(height as f32)
}

pub fn font_from_style<'res>(resources: &'res Resources, style: &Style) -> &'res Font<'res> {
    let config = resources.get_config();
    let strong = style.contains(&Class::Strong);
    let emphasis = style.contains(&Class::Emphasis);

    if style.contains(&Class::Code) {
        resources
            .get_font(&config.mono_font)
            .expect("All fonts should be loaded, or program should've quit")
    } else if strong && emphasis {
        resources
            .get_font(&config.bold_italic_font)
            .expect("All fonts should be loaded, or program should've quit")
    } else if strong {
        resources
            .get_font(&config.bold_font)
            .expect("All fonts should be loaded, or program should've quit")
    } else if emphasis {
        resources
            .get_font(&config.italic_font)
            .expect("All fonts should be loaded, or program should've quit")
    } else {
        resources
            .get_font(&config.default_font)
            .expect("All fonts should be loaded, or program should've quit")
    }
}

pub fn scale_from_style(config: &Config, style: &Style) -> Scale {
    if style.contains(&Class::Heading(4)) {
        config.h4_font_size
    } else if style.contains(&Class::Heading(3)) {
        config.h3_font_size
    } else if style.contains(&Class::Heading(2)) {
        config.h2_font_size
    } else if style.contains(&Class::Heading(1)) {
        config.h1_font_size
    } else {
        config.default_font_size
    }
}

pub fn slice_cow_from_idx<'c>(text: &Cow<'c, str>, idx: usize) -> Cow<'c, str> {
    match text {
        Cow::Owned(string) => Cow::Owned(String::from(&string[idx..])),
        Cow::Borrowed(stringref) => Cow::Borrowed(&stringref[idx..]),
    }
}

pub fn slice_cow_till_idx<'c>(text: &Cow<'c, str>, idx: usize) -> Cow<'c, str> {
    match text {
        Cow::Owned(string) => Cow::Owned(String::from(&string[..idx])),
        Cow::Borrowed(stringref) => Cow::Borrowed(&stringref[..idx]),
    }
}
