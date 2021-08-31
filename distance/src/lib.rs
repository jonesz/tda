// distance/src/lib.rs
mod backend;

use backend::backend::{native_handler, Backend};
use std::ops::{Mul, Sub};

pub enum MetricFn {
    Euclidean,
    Manhattan,
}

pub trait Metric<T, Rhs = Self> {
    fn dist(&self, rhs: &Rhs, metric_fn: Option<MetricFn>, backend: Option<Backend>) -> f64;
}

impl<T> Metric<T> for Vec<T>
where
    T: Sub<Output = T> + Mul<Output = T> + Into<f64> + Copy,
{
    fn dist(&self, rhs: &Vec<T>, metric_fn: Option<MetricFn>, backend: Option<Backend>) -> f64 {
        let backend = backend.unwrap_or(Backend::default());
        let metric_fn = metric_fn.unwrap_or(MetricFn::Euclidean);
        match backend {
            Backend::Native => native_handler(self.as_slice(), rhs.as_slice(), metric_fn),
            Backend::WGPU => panic!("Unimplemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean() {
        let a = vec![1, 0];
        assert_eq!(a.dist(&a, None, None), 0.0);

        let b = vec![2, 1];
        let c = (a.dist(&b, None, None) * 10000.0).round() / 10000.0;
        assert_eq!(c, 1.4142);
    }
}
