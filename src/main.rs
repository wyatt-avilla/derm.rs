mod font_utils;
mod image_utils;
mod similarity;
mod traits;
mod visualize;

use image_utils::img_partitions_from;
use similarity::Points;
use traits::Pointify;
use visualize::print_to_console;

use clap::Parser;
use fontdue::Font;
use image::{DynamicImage, GenericImageView, Pixel};
use std::error::Error;

fn match_char<F, T>(img: &DynamicImage, font: &Font, error_calc: F) -> Result<char, Box<dyn Error>>
where
    F: Fn(&Points, &Points) -> T,
    T: Ord + PartialOrd,
{
    let img_points = img
        .pixels()
        .filter(|(_, _, p)| p.channels()[0] < 245)
        .map(|(x, y, _)| -> Result<(u16, u16), Box<dyn Error>> {
            Ok((u16::try_from(x)?, u16::try_from(y)?))
        })
        .collect::<Result<Points, _>>()?;

    Ok(font
        .chars()
        .iter()
        .map(|(c, _)| -> Result<_, Box<dyn Error>> {
            let (metrics, bitmap) = font.rasterize(*c, img.width() as f32);

            let font_points: Points = bitmap
                .to_points(metrics.width)?
                .filter(|(_, _, p)| *p > 100)
                .map(|(x, y, _)| (x, y))
                .collect();

            Ok((*c, error_calc(&img_points, &font_points)))
        })
        .filter_map(std::result::Result::ok)
        .min_by(|(_, t1), (_, t2)| t1.cmp(t2))
        .ok_or(String::from("unable to find minimum"))?
        .0)
}

/// Unicode image renderer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, disable_version_flag=true)]
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

    // Verbose output
    #[clap(short = 'V', long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let img = image::open(&args.image)
        .map_err(|_| format!("unable to open {}", args.image))?
        .grayscale();
    let font = font_utils::search_for_font(&args.font)?;

    let _sub_images = img_partitions_from(&img, 25, 25, false);

    if args.verbose {
        println!("font in use: {}", font.name().expect("font has no name"));

        print_to_console(&img.pixels(), img.width() as usize, |(_, _, p)| {
            p.channels()[0] < 245
        });
    }

    let closest_char = match_char(&img, &font, similarity::hamming_distance)?;

    let (metrics, bitmap) = font.rasterize(closest_char, 44.0);
    if args.verbose {
        print_to_console(&bitmap.iter(), metrics.width, |&x| x > 100);
    }
    println!("matched character for image: {closest_char}");

    Ok(())
}
