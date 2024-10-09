mod font_utils;
mod image_utils;
mod similarity;
mod traits;
mod visualize;

use image_utils::img_partitions_from;
use itertools::Itertools;
use similarity::Points;
use traits::Pointify;
use visualize::print_to_console;

use clap::Parser;
use fontdue::Font;
use image::{DynamicImage, GenericImageView, Pixel, SubImage};

#[derive(clap::ValueEnum, Clone, Default, Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
enum SimilarityMetric {
    /// Hausdorff Distance
    #[default]
    Hausdorff,

    /// Hamming Distance
    Hamming,

    /// Levenshtein Distance
    Levenshtein,
}

fn match_char<F, T, E>(
    img: &SubImage<&DynamicImage>,
    font: &Font,
    error_calc: F,
) -> Result<char, Box<dyn std::error::Error>>
where
    F: Fn(&Points, &Points) -> Result<T, E>,
    T: PartialOrd,
    E: std::fmt::Display,
{
    let img_points = img
        .pixels()
        .filter(|(_, _, p)| p.channels()[0] < 245)
        .map(
            |(x, y, _)| -> Result<(u16, u16), Box<dyn std::error::Error>> {
                Ok((u16::try_from(x)?, u16::try_from(y)?))
            },
        )
        .collect::<Result<Points, _>>()?;

    Ok(font
        .chars()
        .iter()
        .map(|(c, _)| -> Result<_, Box<dyn std::error::Error>> {
            #[allow(clippy::cast_precision_loss)]
            let (metrics, bitmap) = font.rasterize(*c, img.width() as f32);

            let font_points: Points = bitmap
                .to_points(metrics.width)?
                .filter(|(_, _, p)| *p > 100)
                .map(|(x, y, _)| (x, y))
                .collect();

            Ok((
                *c,
                error_calc(&img_points, &font_points)
                    .map_err(|e| format!("Error calculation failed: {e}"))?,
            ))
        })
        .filter_map(std::result::Result::ok)
        .min_by(|(_, t1), (_, t2)| t1.partial_cmp(t2).expect("comparison failed"))
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

    /// similarity metric
    #[arg(short, long, default_value_t, value_enum)]
    similarity_metric: SimilarityMetric,

    // Verbose output
    #[clap(short = 'V', long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let img = image::open(&args.image)
        .map_err(|_| format!("unable to open {}", args.image))?
        .grayscale();
    let font = font_utils::search_for_font(&args.font)?;

    if args.verbose {
        println!("similarity metric {:?}", args.similarity_metric);
        println!("font in use: {}", font.name().expect("font has no name"));

        print_to_console(&img.pixels(), img.width() as usize, |(_, _, p)| {
            p.channels()[0] < 245
        });
    }

    let keep_partials = false;

    let sub_images = img_partitions_from(
        &img,
        args.pixels_per_char.into(),
        args.pixels_per_char.into(),
        keep_partials,
    );

    let closest_chars = sub_images.iter().map(|s| match args.similarity_metric {
        SimilarityMetric::Hausdorff => match_char(s, &font, similarity::hausdorff_distance),
        SimilarityMetric::Hamming => match_char(s, &font, similarity::hamming_distance),
        SimilarityMetric::Levenshtein => match_char(s, &font, |p1, p2| {
            Ok::<usize, Box<dyn std::error::Error>>(similarity::levenshtein_distance(p1, p2))
        }),
    });

    let rowsize = img.width() / u32::from(args.pixels_per_char) + u32::from(keep_partials);

    closest_chars
        .chunks(rowsize as usize)
        .into_iter()
        .map(|c| c.into_iter().map(|p| p.unwrap_or(' ')).join(""))
        .for_each(|r| println!("| {r} |"));

    Ok(())
}
