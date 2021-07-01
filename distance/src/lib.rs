// distance/src/lib.rs

pub enum Metric {
    Euclidean,
    Manhattan,
}

/// Compute the manhattan distance between two f64 vectors.
pub fn manhattan_distance(p: &[f64], q: &[f64]) -> f64 {
    p.iter()
        .zip(q.iter())
        .fold(0.0, |acc, (x, y)| acc + (x - y).abs())
}

/// Compute the euclidean distance between two f64 vectors.
pub fn euclidean_distance(p: &[f64], q: &[f64]) -> f64 {
    p.iter()
        .zip(q.iter())
        .fold(0.0, |acc, (x, y)| acc + ((x - y) * (x - y)))
        .sqrt()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_manhattan() {
        let a = vec![1.0];
        let b = vec![1.0];
        assert_eq!(manhattan_distance(&a, &b), 0.0);

        let a = vec![3.0, 2.0, 1.0];
        let b = vec![1.0, 2.0, 3.0];
        assert_eq!(manhattan_distance(&a, &b), 4.0);

        let a = vec![3.4, 2.1, 6.6];
        let b = vec![-3.2, -55.0, 100.0];
        assert_eq!(manhattan_distance(&a, &b), 157.10000000000002);
    }

    #[test]
    fn test_euclidean() {
        let a = vec![1.0];
        let b = vec![1.0];
        assert_eq!(euclidean_distance(&a, &b), 0.0);

        let a = vec![3.0, 2.0, 1.0];
        let b = vec![1.0, 2.0, 3.0];
        assert_eq!(euclidean_distance(&a, &b), 2.8284271247461903);

        let a = vec![3.4, 2.1, 6.6];
        let b = vec![-3.2, -55.0, 100.0];
        assert_eq!(euclidean_distance(&a, &b), 109.67009619764178);
    }
}
