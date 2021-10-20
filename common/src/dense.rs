// src/common/src/dense.rs
//! DenseMatrix utilities.
use crate::{Matrix, MatrixOps};

// TODO: Compute array indexing everywhere via a single macro.

/// A matrix defined as rows, columns, and a backing vector.
#[derive(Debug)]
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
        if let Some(elem) = self.2.get_mut(self.1 * r + c) {
            *elem = val;
        } else {
            panic!("call to set out of bounds!");
        }
    }

    fn get(&self, r: usize, c: usize) -> Option<&T> {
        self.2.get(self.1 * r + c)
    }
}

// TODO: This has been a problem area since the 50s; there's likely components
// in the ISA that are useful.
// TODO: These can be implemented via multiplication by specific matrices;
// is that faster?
impl<T> MatrixOps<T> for DenseMatrix<T>
where
    T: std::ops::Add<Output = T> + Copy + Clone,
{
    fn col_swap(&mut self, x: usize, y: usize) {
        for r in 0..self.0 {
            let v1_index = self.1 * r + x;
            let v2_index = self.1 * r + y;
            self.2.swap(v1_index, v2_index);
        }
    }

    // TODO: We're a row-oriented matrix; this can be accomplished by copying
    // contigious blocks of memory. What does the compiler produce?
    fn row_swap(&mut self, x: usize, y: usize) {
        for c in 0..self.1 {
            let v1_index = self.1 * x + c;
            let v2_index = self.1 * y + c;
            self.2.swap(v1_index, v2_index);
        }
    }

    /// Add col 'x' to col 'y'.
    fn col_add(&mut self, x: usize, y: usize) {
        let s = self.2.as_mut_slice();
        for i in 0..self.0 {
            s[y + self.1 * i] = s[y + self.1 * i] + s[x + self.1 * i];
        }
    }

    /// Add row 'x' to col 'y'.
    fn row_add(&mut self, x: usize, y: usize) {
        let v1_index = self.1 * x;
        let v2_index = self.1 * y;

        let s = self.2.as_mut_slice();
        for i in 0..self.1 {
            s[v2_index + i] = s[v2_index + i] + s[v1_index + i];
        }
    }
}

// TODO: There are a myriad of test cases that could remove LOC
// through this.
impl<T> PartialEq for DenseMatrix<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self.0 == other.0, self.1 == other.1) {
            (true, true) => self.2.eq(&other.2),
            _ => false,
        }
    }
}

impl<T> Eq for DenseMatrix<T> where T: PartialEq {}

// TODO: Implement an Iterator?

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

    #[test]
    fn test_col_swap() {
        let mut dm = DenseMatrix::<usize>::new(4, 4);
        for i in 0..4 {
            for j in 0..4 {
                dm.set(i, j, i + j);
            }
        }

        // 0 1 2 3
        // 1 2 3 4
        // 2 3 4 5
        // 3 4 5 6

        dm.col_swap(0, 1);

        for i in 0..4 {
            // 1 2 3 4
            assert_eq!(dm.get(i, 0).is_some(), true);
            assert_eq!(dm.get(i, 0).unwrap(), &(i + 1));

            // 0 1 2 3
            assert_eq!(dm.get(i, 1).is_some(), true);
            assert_eq!(dm.get(i, 1).unwrap(), &i);

            // 2 3 4 5
            assert_eq!(dm.get(i, 2).is_some(), true);
            assert_eq!(dm.get(i, 2).unwrap(), &(i + 2));

            // 3 4 5 6
            assert_eq!(dm.get(i, 3).is_some(), true);
            assert_eq!(dm.get(i, 3).unwrap(), &(i + 3));
        }
    }

    #[test]
    fn test_col_swap_non_square() {
        let mut dm = DenseMatrix::<usize>::new(3, 4);
        for i in 0..3 {
            for j in 0..4 {
                dm.set(i, j, i + j);
            }
        }

        // 0 1 2 3
        // 1 2 3 4
        // 2 3 4 5

        for i in 0..4 {
            // 0 1 2 3
            assert_eq!(dm.get(0, i).is_some(), true);
            assert_eq!(dm.get(0, i).unwrap(), &i);

            // 1 2 3 4
            assert_eq!(dm.get(1, i).is_some(), true);
            assert_eq!(dm.get(1, i).unwrap(), &(i + 1));

            // 2 3 4 5
            assert_eq!(dm.get(2, i).is_some(), true);
            assert_eq!(dm.get(2, i).unwrap(), &(i + 2));
        }

        dm.col_swap(0, 2);

        // 2 1 0 3
        // 3 2 1 4
        // 4 3 2 5

        for i in 0..3 {
            // 2 3 4
            assert_eq!(dm.get(i, 0).is_some(), true);
            assert_eq!(dm.get(i, 0).unwrap(), &(i + 2));

            // 1 2 3
            assert_eq!(dm.get(i, 1).is_some(), true);
            assert_eq!(dm.get(i, 1).unwrap(), &(i + 1));

            // 0 1 2
            assert_eq!(dm.get(i, 2).is_some(), true);
            assert_eq!(dm.get(i, 2).unwrap(), &i);

            // 3 4 5
            assert_eq!(dm.get(i, 3).is_some(), true);
            assert_eq!(dm.get(i, 3).unwrap(), &(i + 3));
        }
    }

    #[test]
    fn test_row_swap() {
        let mut dm = DenseMatrix::<usize>::new(4, 4);
        for i in 0..4 {
            for j in 0..4 {
                dm.set(j, i, i + j);
            }
        }

        // 0 1 2 3
        // 1 2 3 4
        // 2 3 4 5
        // 3 4 5 6

        dm.row_swap(0, 1);

        for i in 0..4 {
            // 1 2 3 4
            assert_eq!(dm.get(0, i).is_some(), true);
            assert_eq!(dm.get(0, i).unwrap(), &(i + 1));

            // 0 1 2 3
            assert_eq!(dm.get(1, i).is_some(), true);
            assert_eq!(dm.get(1, i).unwrap(), &i);

            // 2 3 4 5
            assert_eq!(dm.get(2, i).is_some(), true);
            assert_eq!(dm.get(2, i).unwrap(), &(i + 2));

            // 3 4 5 6
            assert_eq!(dm.get(3, i).is_some(), true);
            assert_eq!(dm.get(3, i).unwrap(), &(i + 3));
        }
    }

    #[test]
    fn test_col_add() {
        let mut dm = DenseMatrix::<usize>::new(4, 4);
        for i in 0..4 {
            for j in 0..4 {
                dm.set(j, i, i + j);
            }
        }

        // 0 1 2 3
        // 1 2 3 4
        // 2 3 4 5
        // 3 4 5 6

        dm.col_add(0, 1);

        for i in 0..4 {
            // 0 1 2 3
            assert_eq!(dm.get(i, 0).is_some(), true);
            assert_eq!(dm.get(i, 0).unwrap(), &i);

            // 1 3 5 7
            assert_eq!(dm.get(i, 1).is_some(), true);
            assert_eq!(dm.get(i, 1).unwrap(), &(i * 2 + 1));

            // 2 3 4 5
            assert_eq!(dm.get(i, 2).is_some(), true);
            assert_eq!(dm.get(i, 2).unwrap(), &(i + 2));

            // 3 4 5 6
            assert_eq!(dm.get(i, 3).is_some(), true);
            assert_eq!(dm.get(i, 3).unwrap(), &(i + 3));
        }
    }

    #[test]
    fn test_row_add() {
        let mut dm = DenseMatrix::<usize>::new(4, 4);
        for i in 0..4 {
            for j in 0..4 {
                dm.set(j, i, i + j);
            }
        }

        // 0 1 2 3
        // 1 2 3 4
        // 2 3 4 5
        // 3 4 5 6

        dm.row_add(0, 1);

        for i in 0..4 {
            // 0 1 2 3
            assert_eq!(dm.get(0, i).is_some(), true);
            assert_eq!(dm.get(0, i).unwrap(), &i);

            // 1 3 5 7
            assert_eq!(dm.get(1, i).is_some(), true);
            assert_eq!(dm.get(1, i).unwrap(), &(i * 2 + 1));

            // 2 3 4 5
            assert_eq!(dm.get(2, i).is_some(), true);
            assert_eq!(dm.get(2, i).unwrap(), &(i + 2));

            // 3 4 5 6
            assert_eq!(dm.get(3, i).is_some(), true);
            assert_eq!(dm.get(3, i).unwrap(), &(i + 3));
        }
    }
}
