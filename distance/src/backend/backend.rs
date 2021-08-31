// distance/src/backend/backend.rs
use crate::backend::native;
use crate::MetricFn;

use std::ops::{Mul, Sub};

#[derive(Clone, Copy)]
pub enum Backend {
    Native,
    WGPU,
}

impl Default for Backend {
    fn default() -> Self {
        Backend::Native
    }
}

pub fn native_handler<T>(a: &[T], b: &[T], metric_fn: MetricFn) -> f64
where
    T: Sub<Output = T> + Mul<Output = T> + Into<f64> + Copy,
{
    match metric_fn {
        MetricFn::Euclidean => native::euclidean_dist(a, b),
        MetricFn::Manhattan => native::manhattan_dist(a, b),
    }
}
