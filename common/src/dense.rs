// src/common/src/dense.rs
//! DenseMatrix utilities.
use crate::Matrix;

/// A matrix defined as rows, columns, and a backing vector.
pub struct DenseMatrix<T>(usize, usize, Vec<T>);

impl<T> Matrix<T> for DenseMatrix<T> {
    fn new(r: usize, c: usize) -> Self {
        let mut buf = Vec::with_capacity(r * c);
        unsafe {
            buf.set_len(r * c);
        }

        DenseMatrix(r, c, buf)
    }

    /// (rows, cols).
    fn dim(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    fn set(&mut self, r: usize, c: usize, val: T) {
        if let Some(elem) = self.2.get_mut(self.0 * c + r) {
            *elem = val;
        } else {
            panic!("call to set out of bounds!");
        }
    }

    fn get(&self, r: usize, c: usize) -> Option<&T> {
        self.2.get(self.0 * c + r)
    }
}

// TODO: Implement an Iterator, swap operations?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dense_matrix() {
        let mut dm = DenseMatrix::<bool>::new(10, 10);
        assert_eq!(dm.dim(), (10, 10));

        dm.set(0, 0, false);
        assert_eq!(dm.get(0, 0), Some(&false));
        dm.set(1, 1, true);
        assert_eq!(dm.get(1, 1), Some(&true));
    }
}
