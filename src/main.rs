use std::error::Error;

use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::error::GlyphLoadingError;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use font_kit::source::SystemSource;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};

use font_kit::handle::Handle;

fn print_char(font: &Font, glyph_id: u32) -> Result<(), GlyphLoadingError> {
    println!("{glyph_id} ↓");
    let special_number = 44; // in the example snippet, idk yet
    let mut canvas = Canvas::new(Vector2I::splat(special_number), Format::A8);
    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        special_number as f32,
        // x, y transform
        Transform2F::from_translation(Vector2F::new(0.0, special_number as f32 - 5.0)),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,
    )?;
    for (i, val) in canvas.pixels.iter().enumerate() {
        if i % special_number as usize == 0 {
            print!("\n");
        } else {
            print!("{val:<3}");
        }
    }
    print!("\n");

    Ok(())
}

fn search_for_font(family_name: &str, font_name_pattern: &str) -> Result<Font, Box<dyn Error>> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let hask = search_for_font(
        "Hasklug Nerd Font Mono",
        "Hasklug Medium Nerd Font Complete Mono",
    )?;

    let jp = search_for_font("Noto Serif CJK JP", "Regular")?;
    println!("{} contains {} chars", hask.full_name(), hask.glyph_count());
    println!("{} contains {} chars", jp.full_name(), jp.glyph_count());

    print_char(&hask, hask.glyph_for_char('█').unwrap())?;
    print_char(&jp, jp.glyph_for_char('漢').unwrap())?;

    Ok(())
}
