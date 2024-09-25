mod font_utils;
mod image_utils;
mod similarity;
mod visualize;

use font_utils::search_for_font;
use image_utils::img_partitions_from;

use fontdue::Font;
use image::{GenericImageView, Pixel};
use std::error::Error;
use visualize::print_to_console;

fn main() -> Result<(), Box<dyn Error>> {
    let font_data = std::fs::read("/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc")?;

    let jp = Font::from_bytes(font_data, fontdue::FontSettings::default())?;
    let (metrics, bitmap) = jp.rasterize('漢', 44.0);

    println!("font in use: {}", jp.name().expect("font has no name"));

    print_to_console(&bitmap.iter(), metrics.width, |&x| x > 100);

    let img = image::open("src/images/smiley.png")?;
    let _sub_images = img_partitions_from(&img, 25, 25, false);

    print_to_console(
        &img.grayscale().pixels(),
        img.width() as usize,
        |(_, _, p)| p.channels()[0] < 245,
    );

    Ok(())
}
