use super::Points;

use itertools::Itertools;

pub fn hamming_distance(a: &Points, b: &Points) -> usize {
    a.symmetric_difference(b).count()
}
