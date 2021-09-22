// distance/src/lib.rs
//! Utilities for computing distance between vectors.
mod backend;

use backend::backend::{native_handler, Backend};
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
    fn dist(&self, rhs: &Rhs, metric_fn: MetricFn, backend: Option<Backend>) -> Self::Output;
}

impl<T> Metric<T> for Vec<T>
where
    T: Sub<Output = T> + Mul<Output = T> + Into<f64> + Copy,
{
    type Output = f64;

    fn dist(&self, rhs: &Vec<T>, metric_fn: MetricFn, backend: Option<Backend>) -> Self::Output {
        let backend = backend.unwrap_or(Backend::default());
        match backend {
            Backend::Native => native_handler(self.as_slice(), rhs.as_slice(), metric_fn),

            // TODO: This probably isn't the correct way to do this; the
            // current `point_cloud` implementation is to call this a.dist(b)
            // function where a and b are both vectors. What ends up happening
            // (if this was implemented) is multiple data transfers with the
            // GPU when it would be much more efficient to coalesce into
            // a single transaction.
            Backend::WGPU => panic!("Unimplemented"),
        }
    }
}

/// Given some point cloud data, produce a distance matrix between each point.
pub fn point_cloud_dist<T>(
    data: &Vec<Vec<T>>,
    metric_fn: Option<MetricFn>,
    backend: Option<Backend>,
) -> Vec<Vec<f64>>
where
    T: Sub<Output = T> + Mul<Output = T> + Into<f64> + Copy,
{
    let metric_fn = metric_fn.unwrap_or(MetricFn::default());

    let mut r = Vec::new();
    for x in data {
        let mut tmp = Vec::new();
        for y in data {
            tmp.push(x.dist(y, metric_fn, backend));
        }
        r.push(tmp);
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dist_impl() {
        let a = vec![1, 0];
        assert_eq!(a.dist(&a, MetricFn::Euclidean, None), 0.0);

        let b = vec![2, 1];
        let c = (a.dist(&b, MetricFn::Euclidean, None) * 10000.0).round() / 10000.0;
        assert_eq!(c, 1.4142);
    }

    #[test]
    fn test_point_cloud_dist() {
        let a = vec![vec![0, 0], vec![0, 0]];
        let b = point_cloud_dist(&a, None, None);
        let c = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
        assert_eq!(b, c);

        let a = vec![vec![1, 1], vec![2, 2]];
        let b = point_cloud_dist(&a, Some(MetricFn::Manhattan), None);
        let c = vec![vec![0.0, 2.0], vec![2.0, 0.0]];
        assert_eq!(b, c);
    }
}
