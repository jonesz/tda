// distance/src/lib.rs
//! Utilities for computing distance between vectors.
mod backend;

use backend::backend::{native_handler, Backend};
use common::Matrix;
use std::ops::{Mul, Sub};

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

pub trait Metric<T, Rhs = Self> {
    fn dist(&self, rhs: &Rhs, metric_fn: MetricFn, backend: Option<Backend>) -> f64;
}

impl<T> Metric<T> for Vec<T>
where
    T: Sub<Output = T> + Mul<Output = T> + Into<f64> + Copy,
{
    fn dist(&self, rhs: &Vec<T>, metric_fn: MetricFn, backend: Option<Backend>) -> f64 {
        let backend = backend.unwrap_or(Backend::default());
        match backend {
            Backend::Native => native_handler(self.as_slice(), rhs.as_slice(), metric_fn),
            Backend::WGPU => panic!("Unimplemented"),
        }
    }
}

/// Given some point cloud data, produce a distance matrix between each point.
pub fn point_cloud_dist<T>(
    data: Vec<Vec<T>>,
    metric_fn: Option<MetricFn>,
    backend: Option<Backend>,
) -> Matrix<f64>
where
    T: Sub<Output = T> + Mul<Output = T> + Into<f64> + Copy,
{
    let metric_fn = metric_fn.unwrap_or(MetricFn::default());
    let mut out_mat = Matrix::new(data.len(), data.len());

    // TODO: O(n^2) rather than O(nlogn).
    for (i, a) in data.iter().enumerate() {
        for (j, b) in data.iter().enumerate() {
            out_mat.set(i, j, a.dist(b, metric_fn, backend));
        }
    }

    out_mat
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

    /* TODO: Rewrite this to handle the new matrix type.
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
    */
}
