mod font_utils;
mod image_utils;
mod similarity;
mod traits;
mod visualize;

use image_utils::img_partitions_from;
use traits::Points;
use visualize::print_to_console;

use clap::Parser;
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

/// Unicode image renderer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input image
    #[arg(short, long)]
    image: String,

    /// Font to use during rendering
    #[arg(short, long, default_value_t = String::from("mono"))]
    font: String,

    /// Scale of unicode image
    #[arg(short, long, default_value_t = 50)]
    pixels_per_char: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let img = image::open(&args.image)
        .map_err(|_| format!("unable to open {}", args.image))?
        .grayscale();
    let font = font_utils::search_for_font(&args.font)?;

    let _sub_images = img_partitions_from(&img, 25, 25, false);
    println!("font in use: {}", font.name().expect("font has no name"));

    print_to_console(&img.pixels(), img.width() as usize, |(_, _, p)| {
        p.channels()[0] < 245
    });

    let closest_char = match_char(&img, &font)?;

    let (metrics, bitmap) = font.rasterize(closest_char, 44.0);
    print_to_console(&bitmap.iter(), metrics.width, |&x| x > 100);
    println!("matched character for image: {closest_char}");

    Ok(())
}
