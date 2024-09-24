mod font_utils;
mod image_utils;
mod similarity;

use font_utils::{print_char, search_for_font};
use image_utils::{img_partitions_from, print_image};
use std::error::Error;

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

    let test_img_path = "src/images/smiley.png";

    print_image(test_img_path)?;

    let img = image::open(test_img_path)?;
    let _sub_images = img_partitions_from(&img, 25, 25, false);

    Ok(())
}
