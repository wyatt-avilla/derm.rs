use super::Points;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptySet,
    NoMinimum,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptySet => write!(f, "Hausdorff distance isn't well defined for empty sets"),
            Error::NoMinimum => write!(f, "Unable to find minimum"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::EmptySet => "Hausdorff distance isn't well defined for empty sets",
            Error::NoMinimum => "Unable to find minimum",
        }
    }
}

fn euclidean_distance(x: (u16, u16), y: (u16, u16)) -> f64 {
    f64::from((i32::from(x.0) - i32::from(y.0)).pow(2) + (i32::from(x.1) - i32::from(y.1)).pow(2))
        .sqrt()
}

#[allow(clippy::module_name_repetitions)]
pub fn hausdorff_distance(a: &Points, b: &Points) -> Result<f32> {
    if a.is_empty() || b.is_empty() {
        return Err(Error::EmptySet);
    }

    let maybe_min = a
        .iter()
        .map(|p_1| {
            #[allow(clippy::cast_possible_truncation)]
            b.iter()
                .map(|p_2| euclidean_distance(*p_1, *p_2) as f32)
                .fold(f32::INFINITY, f32::min)
        })
        .fold(f32::NEG_INFINITY, f32::max);

    if maybe_min.is_finite() {
        Ok(maybe_min)
    } else {
        Err(Error::NoMinimum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = f32::EPSILON;

    #[test]
    fn empty_sets() {
        let p1 = Points::new();
        let p2 = Points::new();

        let expected = Err(Error::EmptySet);
        let result = hausdorff_distance(&p1, &p2);

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn single_elements() {
        let p1 = Points::from([(0, 0)]);
        let p2 = Points::from([(0, 0)]);

        let expected = 0.0;
        let result = hausdorff_distance(&p1, &p2).unwrap();

        assert!(
            (result - expected).abs() <= EPSILON,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn different_single_elements() {
        let p1 = Points::from([(0, 0)]);
        let p2 = Points::from([(3, 4)]);

        let expected = 5.0;
        let result = hausdorff_distance(&p1, &p2).unwrap();

        assert!(
            (result - expected).abs() <= EPSILON,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn multiple_identical_points() {
        let p1 = Points::from([(0, 0), (1, 1), (2, 2)]);
        let p2 = Points::from([(0, 0), (1, 1), (2, 2)]);

        let expected = 0.0;
        let result = hausdorff_distance(&p1, &p2).unwrap();

        assert!(
            (result - expected).abs() <= EPSILON,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn non_overlapping_sets() {
        let p1 = Points::from([(0, 0), (1, 1)]);
        let p2 = Points::from([(10, 10), (11, 11)]);

        let expected = 14.142_136;
        let result = hausdorff_distance(&p1, &p2).unwrap();

        assert!(
            (result - expected).abs() <= EPSILON,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn partially_overlapping_sets() {
        let p1 = Points::from([(0, 0), (1, 1), (2, 2)]);
        let p2 = Points::from([(0, 0), (2, 2), (3, 3)]);

        let expected = 2.0_f32.sqrt();
        let result = hausdorff_distance(&p1, &p2).unwrap();

        assert!(
            (result - expected).abs() <= EPSILON,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }
}
