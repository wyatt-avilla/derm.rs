use fontdue::Font;
use std::error::Error;

// TODO: make this more inteligent
pub fn search_for_font(path: &str) -> Result<Font, Box<dyn Error>> {
    let font_data = std::fs::read(path)?;
    Ok(Font::from_bytes(
        font_data,
        fontdue::FontSettings::default(),
    )?)
}
