// distance/src/dist.rs
//! Distance formula implementations.
use std::ops::{Mul, Sub};

/// d(p, q) = sqrt((p1 - q1)^2 + (p2 - q2)^2 + ... + (pn - qn)^2).
pub fn euclidean_dist<T>(a: &[T], b: &[T]) -> f64
where
    T: Sub<Output = T> + Mul<Output = T> + Into<f64> + Copy,
{
    let r = a
        .iter()
        .zip(b.iter())
        .into_iter()
        .fold(0.0, |acc, (x, y)| acc + ((*x - *y) * (*x - *y)).into());
    r.sqrt()
}

/// d(p, q) = Sum {i=1; i=>n} abs(pi - qi).
pub fn manhattan_dist<T>(a: &[T], b: &[T]) -> f64
where
    T: Sub<Output = T> + Mul<Output = T> + Into<f64> + Copy,
{
    a.iter()
        .zip(b.iter())
        .into_iter()
        .fold(0.0, |acc, (x, y)| acc + (*x - *y).into().abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean() {
        let a = vec![1, 0];
        let b = vec![2, 1];
        assert_eq!(euclidean_dist(&a, &a), 0.0);

        let c = (euclidean_dist(&a, &b) * 10000.0).round() / 10000.0;
        assert_eq!(c, 1.4142);
    }

    #[test]
    fn test_manhattan() {
        let a = vec![1, 0];
        let b = vec![2, 1];
        assert_eq!(manhattan_dist(&a, &a), 0.0);

        let c = (manhattan_dist(&a, &b) * 10000.0).round() / 10000.0;
        assert_eq!(c, 2.0);
    }
}
