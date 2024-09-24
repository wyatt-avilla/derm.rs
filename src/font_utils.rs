use font_kit::font::Font;
use font_kit::handle::Handle;
use font_kit::source::SystemSource;
use std::error::Error;

pub fn search_for_font(family_name: &str, font_name_pattern: &str) -> Result<Font, Box<dyn Error>> {
    let src = SystemSource::new();
    let fam = src.select_family_by_name(family_name)?;
    Ok(fam
        .fonts()
        .iter()
        .find(|h| {
            matches!(h, Handle::Path { path, .. } if path
                .to_str()
                .map_or(false, |name| name.contains(font_name_pattern)))
        })
        .ok_or("unable to find font in system")?
        .load()?)
}
