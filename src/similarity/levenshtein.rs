use super::Points;
use core::cmp::max;

#[allow(clippy::module_name_repetitions)]
pub fn levenshtein_distance(a: &Points, b: &Points) -> usize {
    let intersection_cardinality = a.intersection(b).count();

    max(
        a.len() - intersection_cardinality,
        b.len() - intersection_cardinality,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_sets() {
        let p1 = Points::new();
        let p2 = Points::new();

        let expected = 0;
        let result = levenshtein_distance(&p1, &p2);

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn identical_sets() {
        let p1 = Points::from([(0, 0), (1, 1)]);
        let p2 = Points::from([(0, 0), (1, 1)]);

        let expected = 0;
        let result = levenshtein_distance(&p1, &p2);

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn single_point_difference() {
        let p1 = Points::from([(0, 0)]);
        let p2 = Points::from([(1, 1)]);

        let expected = 1;
        let result = levenshtein_distance(&p1, &p2);

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn multiple_differences() {
        let p1 = Points::from([(0, 0), (2, 2), (3, 3)]);
        let p2 = Points::from([(1, 1), (2, 2), (4, 4)]);

        let expected = 2;
        let result = levenshtein_distance(&p1, &p2);

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn different_length_sets() {
        let p1 = Points::from([(0, 0), (2, 2), (3, 3)]);
        let p2 = Points::from([(1, 1)]);

        let expected = 3;
        let result = levenshtein_distance(&p1, &p2);

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn empty_vs_non_empty() {
        let p1 = Points::new();
        let p2 = Points::from([(0, 0), (1, 1)]);

        let expected = 2;
        let result = levenshtein_distance(&p1, &p2);

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn no_common_points() {
        let p1 = Points::from([(0, 0), (2, 2)]);
        let p2 = Points::from([(1, 1), (3, 3)]);

        let expected = 2;
        let result = levenshtein_distance(&p1, &p2);

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn partially_overlapping_sets() {
        let p1 = Points::from([(0, 0), (1, 1), (2, 2)]);
        let p2 = Points::from([(3, 3), (1, 1), (2, 2)]);

        let expected = 1;
        let result = levenshtein_distance(&p1, &p2);

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn partially_overlapping_sets_different_cardinalities() {
        let p1 = Points::from([(0, 0), (1, 1), (2, 2), (4, 4)]);
        let p2 = Points::from([(3, 3), (1, 1), (2, 2)]);

        let expected = 2;
        let result = levenshtein_distance(&p1, &p2);

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn completely_different_points() {
        let p1 = Points::from([(0, 0), (1, 1)]);
        let p2 = Points::from([(2, 2), (3, 3)]);

        let expected = 2;
        let result = levenshtein_distance(&p1, &p2);

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }

    #[test]
    fn multiple_similar_points() {
        let p1 = Points::from([(0, 0), (1, 1), (2, 2), (3, 3)]);
        let p2 = Points::from([(0, 0), (2, 2), (3, 3), (4, 4)]);

        let expected = 1;
        let result = levenshtein_distance(&p1, &p2);

        assert_eq!(
            expected, result,
            "Expected: {expected:?}, but got: {result:?}"
        );
    }
}
