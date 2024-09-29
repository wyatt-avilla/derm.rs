use super::Points;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    Cardinality(usize, usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Cardinality(i, j) => write!(
                f,
                "Cannot compute Hamming distance between sets with cardinality {i} and {j}"
            ),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Cardinality(_, _) => {
                "Hamming distance isn't well defined for sets with different cardinalities"
            }
        }
    }
}

pub fn hamming_distance(a: &Points, b: &Points) -> Result<usize> {
    if a.len() != b.len() {
        return Err(Error::Cardinality(a.len(), b.len()));
    }

    Ok(a.symmetric_difference(b).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_sets() {
        let p1 = Points::new();
        let p2 = Points::new();

        let expected = 0;
        let result = hamming_distance(&p1, &p2).unwrap();

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn identical_sets() {
        let p1 = Points::from([(0, 0)]);
        let p2 = Points::from([(0, 0)]);

        let expected = 0;
        let result = hamming_distance(&p1, &p2).unwrap();

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn different_single_point_sets() {
        let p1 = Points::from([(0, 0)]);
        let p2 = Points::from([(1, 1)]);

        let expected = 2;
        let result = hamming_distance(&p1, &p2).unwrap();

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn partially_identical_sets() {
        let p1 = Points::from([(0, 0), (1, 1)]);
        let p2 = Points::from([(0, 0), (2, 2)]);

        let expected = 2;
        let result = hamming_distance(&p1, &p2).unwrap();

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn larger_identical_sets() {
        let p1 = Points::from([(0, 0), (1, 1), (2, 2)]);
        let p2 = Points::from([(0, 0), (1, 1), (2, 2)]);

        let expected = 0;
        let result = hamming_distance(&p1, &p2).unwrap();

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn sets_with_extra_points() {
        let p1 = Points::from([(0, 0), (1, 1)]);
        let p2 = Points::from([(0, 0), (1, 1), (2, 2)]);

        let expected = Error::Cardinality(2, 3);
        let result = hamming_distance(&p1, &p2).unwrap_err();

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn completely_different_sets() {
        let p1 = Points::from([(0, 0), (1, 1)]);
        let p2 = Points::from([(2, 2), (3, 3)]);

        let expected = 4;
        let result = hamming_distance(&p1, &p2).unwrap();

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }
}
