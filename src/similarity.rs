use std::collections::HashSet;

pub type Points = HashSet<(u16, u16)>;

mod hamming;
mod hausdorff;

pub use hamming::hamming_distance;
pub use hausdorff::hausdorff_distance;
