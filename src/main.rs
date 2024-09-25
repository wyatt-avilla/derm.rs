mod font_utils;
mod image_utils;
mod similarity;
mod traits;
mod visualize;

use image_utils::img_partitions_from;
use traits::Points;
use visualize::print_to_console;

use fontdue::Font;
use image::{DynamicImage, GenericImageView, Pixel};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

fn match_char(img: &DynamicImage, font: &Font) -> Result<char, Box<dyn Error>> {
    let img_points = img
        .pixels()
        .filter(|(_, _, p)| p.channels()[0] < 245)
        .map(|(x, y, _)| -> Result<(u16, u16), Box<dyn Error>> {
            Ok((u16::try_from(x)?, u16::try_from(y)?))
        })
        .collect::<Result<HashSet<_>, _>>()?;

    let mut dist_map: HashMap<char, f32> = HashMap::new();

    let mut best_dist = f32::INFINITY;
    let mut best_char = 'x';

    for (c, _) in font.chars() {
        let (metrics, bitmap) = font.rasterize(*c, img.width() as f32);

        if metrics.width == 0 {
            continue;
        }

        let font_points: HashSet<_> = bitmap
            .to_points(metrics.width)?
            .filter(|(_, _, p)| *p > 100)
            .map(|(x, y, _)| (x, y))
            .collect();

        let curr_dist = similarity::hausdorff::distance(&img_points, &font_points);
        dist_map.insert(*c, curr_dist);

        if curr_dist < best_dist {
            best_dist = curr_dist;
            best_char = *c;
        }
    }

    Ok(best_char)
}

fn main() -> Result<(), Box<dyn Error>> {
    let font_data = std::fs::read("/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc")?;

    let jp = Font::from_bytes(font_data, fontdue::FontSettings::default())?;
    println!("font in use: {}", jp.name().expect("font has no name"));

    let (metrics, bitmap) = jp.rasterize('䯁', 44.0);
    print_to_console(&bitmap.iter(), metrics.width, |&x| x > 100);

    let img = image::open("src/images/smiley.png")?.grayscale();
    let _sub_images = img_partitions_from(&img, 25, 25, false);

    print_to_console(&img.pixels(), img.width() as usize, |(_, _, p)| {
        p.channels()[0] < 245
    });

    println!("matched character for image: {}", match_char(&img, &jp)?);

    Ok(())
}
