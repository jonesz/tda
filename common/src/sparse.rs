// src/common/src/sparse.rs
//! SparseMatrix utilities.
use crate::Matrix;
use std::collections::HashMap;

/// A matrix defined as rows, columns, and a backing hashmap.
pub struct SparseMatrix<T>(usize, usize, HashMap<(usize, usize), T>);

impl<T> Matrix<T> for SparseMatrix<T> {
    fn new(rows: usize, cols: usize) -> Self {
        SparseMatrix(rows, cols, HashMap::new())
    }

    /// (rows, cols).
    fn dim(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    fn set(&mut self, r: usize, c: usize, val: T) {
        self.2.insert((r, c), val);
    }

    // TODO: Rather than returning 'None', the T::default() value
    // could be instantiated. In the majority of cases the type
    // parameter will be a boolean, which works out to be 'false'.
    fn get(&self, r: usize, c: usize) -> Option<&T> {
        self.2.get(&(r, c))
    }
}

// TODO: Implement an Iterator, swap operations?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparse_matrix() {
        let mut dm = SparseMatrix::<bool>::new(10, 10);
        assert_eq!(dm.dim(), (10, 10));

        dm.set(0, 0, true);
        assert_eq!(dm.get(0, 0), Some(&true));
        assert_eq!(dm.get(1, 1), None);
    }
}
