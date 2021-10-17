// src/distance/src/point_cloud.rs
use crate::{Metric, MetricFn};
/// Utilities for producing distance matrices from point cloud data.
use common::{dense::DenseMatrix, Matrix};

/// Produce a NxN distance matrix.
pub fn to_dist_mat(cloud: &[Vec<f64>], metric: Option<MetricFn>) -> DenseMatrix<f64> {
    let mut dm = DenseMatrix::new(cloud.len(), cloud.len());

    for (i, a) in cloud.iter().enumerate() {
        for (j, b) in cloud.iter().enumerate() {
            let d = a.dist(b, metric);
            dm.set(i, j, d);
        }
    }

    dm
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_to_dist_mat() {
        let mut cloud: Vec<Vec<f64>> = vec![];
        for i in 0..100 {
            // Each vector K is smaller than the vector K + 1.
            cloud.push((0..100).map(|x| (x as f64) * (i as f64)).collect());
        }

        let mat = to_dist_mat(&cloud, None);
        let (cols, rows) = mat.dim();
        assert_eq!(cols, 100);
        assert_eq!(rows, 100);

        for i in 0..cols {
            for j in 0..rows {
                // Should always return a value.
                assert_eq!(mat.get(j, i).is_some(), true);

                // Each vector is different; i == j should be 0 distance,
                // otherwise the distance resides in P.
                match i.cmp(&j) {
                    Ordering::Equal => assert_eq!(mat.get(j, i).unwrap() == &0.0, true),
                    _ => assert_eq!(mat.get(j, i).unwrap() > &0.0, true),
                }
            }
        }
    }
}
