use std::error::Error;

use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use font_kit::source::SystemSource;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};

use font_kit::handle::Handle;

use image::{GenericImageView, Pixel};

fn print_image(path_str: &str) -> Result<(), Box<dyn Error>> {
    let img = image::open(path_str)?;
    println!("dimensions {:?}x{:?}", img.width(), img.height());

    for (i, (_x, _y, channels)) in img.grayscale().pixels().enumerate() {
        let rgb = channels.to_rgb();
        let pixel_value = rgb.channels()[0];

        if pixel_value > 245 {
            print!(" . ");
        } else {
            print!("{pixel_value}");
        }

        if i % img.width() as usize == 0 {
            println!();
        }
    }

    Ok(())
}

fn print_char(font: &Font, glyph_id: u32, canvas_size: u16) -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new(Vector2I::splat(i32::from(canvas_size)), Format::A8);

    let vert_trans_fact = f32::from(canvas_size) * 0.10;
    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        f32::from(canvas_size),
        Transform2F::from_translation(Vector2F::new(0.0, f32::from(canvas_size) - vert_trans_fact)),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,
    )?;

    println!("{glyph_id} ↓");
    for (i, val) in canvas.pixels.iter().enumerate() {
        if i % canvas_size as usize == 0 {
            println!();
        } else {
            print!("{val:<3}");
        }
    }
    println!();

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

    print_char(&hask, hask.glyph_for_char('█').unwrap(), 44)?;
    print_char(&jp, jp.glyph_for_char('漢').unwrap(), 44)?;

    print_image("src/images/smiley.png")?;

    Ok(())
}
