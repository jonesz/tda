// distance/src/lib.rs
//! Utilities for computing distance.
mod dist;
pub mod neighborhood;
pub mod point_cloud;
use std::ops::{Mul, Sub};

/// Enumeration for each specific distance formula.
#[derive(Clone, Copy)]
pub enum MetricFn {
    Euclidean,
    Manhattan,
}

impl Default for MetricFn {
    fn default() -> Self {
        MetricFn::Euclidean
    }
}

/// A type implementing the Metric trait allows for computation of distance
/// via a.dist(b). Requires the specification of an output type.
pub trait Metric<T, Rhs = Self> {
    type Output;

    /// Compute the distance between two values of the same type.
    fn dist(&self, rhs: &Rhs, metric_fn: Option<MetricFn>) -> Self::Output;
}

// No need for specialization apparently, see benches/f64_specialization.rs
impl<T> Metric<T> for Vec<T>
where
    T: Sub<Output = T> + Mul<Output = T> + Into<f64> + Copy,
{
    type Output = f64;

    fn dist(&self, rhs: &Vec<T>, metric_fn: Option<MetricFn>) -> Self::Output {
        let metric_fn = metric_fn.unwrap_or_default();
        match metric_fn {
            MetricFn::Euclidean => dist::euclidean_dist(self, rhs),
            MetricFn::Manhattan => dist::manhattan_dist(self, rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dist_impl() {
        let a = vec![1, 0];
        assert_eq!(a.dist(&a, Some(MetricFn::Euclidean)), 0.0);

        let b = vec![2, 1];
        let c = (a.dist(&b, Some(MetricFn::Euclidean)) * 10000.0).round() / 10000.0;
        assert_eq!(c, 1.4142);
    }
}
