use image::{DynamicImage, GenericImageView, SubImage};
use std::vec::Vec;

// row major traversal
pub fn img_partitions_from(
    img: &DynamicImage,
    partition_width: u32,
    partition_height: u32,
    keep_partial_partitions: bool,
) -> Vec<SubImage<&DynamicImage>> {
    let (img_width, img_height) = img.dimensions();

    if partition_width >= img_width || partition_height >= img_height {
        return vec![img.view(0, 0, img_width, img_height)];
    }

    let partition_start_coordinates = (0..img_height)
        .step_by(partition_height as usize)
        .flat_map(|y| {
            (0..img_width)
                .step_by(partition_width as usize)
                .map(move |x| (x, y))
        })
        .filter(|&(x, y)| {
            keep_partial_partitions
                || (img_width - x >= partition_width && img_height - y >= partition_height)
        });

    partition_start_coordinates
        .into_iter()
        .filter_map(|(x, y)| {
            if x + partition_width <= img_width && y + partition_height <= img_height {
                return Some(img.view(x, y, partition_width, partition_height));
            }

            if keep_partial_partitions {
                let truncated_width = if x + partition_width > img_width {
                    img_width - x
                } else {
                    partition_width
                };
                let truncated_height = if y + partition_height > img_height {
                    img_height - y
                } else {
                    partition_height
                };

                return Some(img.view(x, y, truncated_width, truncated_height));
            }

            None
        })
        .collect()
}
