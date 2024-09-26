use std::collections::HashSet;

fn euclidean_distance(x: (u16, u16), y: (u16, u16)) -> f64 {
    f64::from((i32::from(x.0) - i32::from(y.0)).pow(2) + (i32::from(x.1) - i32::from(y.1)).pow(2))
        .sqrt()
}

pub fn hausdorff_distance(sx: &HashSet<(u16, u16)>, sy: &HashSet<(u16, u16)>) -> f32 {
    sx.iter()
        .map(|p_1| {
            #[allow(clippy::cast_possible_truncation)]
            sy.iter()
                .map(|p_2| euclidean_distance(*p_1, *p_2) as f32)
                .fold(f32::INFINITY, f32::min)
        })
        .fold(f32::NEG_INFINITY, f32::max)
}
