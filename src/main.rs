mod font_utils;
mod image_utils;
mod similarity;
mod visualize;

use font_utils::search_for_font;
use image_utils::img_partitions_from;

use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::hinting::HintingOptions;
use image::{GenericImageView, Pixel};
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use std::error::Error;
use visualize::print_to_console;

fn main() -> Result<(), Box<dyn Error>> {
    let hask = search_for_font(
        "Hasklug Nerd Font Mono",
        "Hasklug Medium Nerd Font Complete Mono",
    )?;

    let jp = search_for_font("Noto Serif CJK JP", "Regular")?;
    println!("{} contains {} chars", hask.full_name(), hask.glyph_count());
    println!("{} contains {} chars", jp.full_name(), jp.glyph_count());

    let canvas_size: u16 = 44;
    let mut canvas = Canvas::new(Vector2I::splat(i32::from(canvas_size)), Format::A8);
    let vert_trans_fact = f32::from(canvas_size) * 0.10;
    jp.rasterize_glyph(
        &mut canvas,
        jp.glyph_for_char('漢').unwrap(),
        f32::from(canvas_size),
        Transform2F::from_translation(Vector2F::new(0.0, f32::from(canvas_size) - vert_trans_fact)),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,
    )?;

    print_to_console(&canvas.pixels.iter(), canvas_size as usize, |&x| x > 100);

    let img = image::open("src/images/smiley.png")?;
    let _sub_images = img_partitions_from(&img, 25, 25, false);

    print_to_console(
        &img.grayscale().pixels(),
        img.width() as usize,
        |(_, _, p)| p.channels()[0] < 245,
    );

    Ok(())
}
