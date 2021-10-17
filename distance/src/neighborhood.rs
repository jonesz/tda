// src/complex/src/neighborhood.rs
//! Utilities for computing neighborhood graphs.
use common::{sparse::SparseMatrix, Matrix};

/// Compute an adjacency matrix from a distance matrix and an epsilon value.
pub fn to_adjacency<M: Matrix<f64>>(dist: &M, epsilon: f64) -> SparseMatrix<bool> {
    let (rows, cols) = dist.dim();
    let mut sm = SparseMatrix::new(rows, cols);

    for c in 0..cols {
        for r in 0..rows {
            if let Some(val) = dist.get(r, c) {
                if *val < epsilon {
                    sm.set(r, c, true);
                }
            }
        }
    }

    sm
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::dense::DenseMatrix;

    #[test]
    fn test_to_adjacency() {
        let mut dm = DenseMatrix::new(10, 10);
        for c in 0..10 {
            for r in 0..10 {
                dm.set(r, c, 10.0);
            }
        }

        // Should be an adjacent matrix of all None/false.
        let adj_f = to_adjacency(&dm, 10.0);
        // Should be an adjacent matrix of all true.
        let adj_t = to_adjacency(&dm, 20.0);

        for c in 0..10 {
            for r in 0..10 {
                assert_eq!(adj_f.get(r, c).is_none(), true);
                assert_eq!(adj_t.get(r, c).is_some(), true);
                assert_eq!(adj_t.get(r, c).unwrap(), &true);
            }
        }
    }
}
