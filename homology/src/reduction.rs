// homology/src/reduction.rs
//! Transform Matrices into Smith-Normal form.
//! Referenced "Computational Topology An Introduction" by Edelsbrunner and
//! Harer. ISBN 978-0-8218-4925-5
use common::{Matrix, MatrixOps};

/// The reduction algorithm mod 2 presented on pg. 88.
pub fn reduce_z2<M>(mat: &mut M, x: usize)
where
    M: Matrix<usize> + MatrixOps<usize>,
{
    let (rows, cols) = mat.dim();
    // a:1,1;    a:2,1;    ...; a:np,1
    // a:1,2;    a:2,2;    ...; a:np,2
    // a:1,np-1; a:2,np-1; ...; a:np,np-1

    for k in x..rows {
        for l in x..cols {
            if let Some(v) = mat.get(k, l) {
                if v % 2 == 1 {
                    mat.row_swap(x, k);
                    mat.col_swap(x, l);

                    for i in x + 1..rows {
                        if let Some(v) = mat.get(i, x) {
                            if v % 2 == 1 {
                                mat.row_add(x, i);
                            }
                        }
                    }

                    for j in x + 1..cols {
                        if let Some(v) = mat.get(x, j) {
                            if v % 2 == 1 {
                                mat.col_add(x, j);
                            }
                        }
                    }

                    reduce_z2(mat, x + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::dense::DenseMatrix;

    #[test]
    fn test_reduce_case_1() {
        let mut mat = DenseMatrix::<usize>::new(4, 6);

        // https://courses.cs.duke.edu//fall06/cps296.1/Lectures/sec-IV-3.pdf
        // 1 1 1 0 0 0
        // 1 0 0 1 1 0
        // 0 1 0 1 0 1
        // 0 0 1 0 1 1
        mat.set(0, 0, 1);
        mat.set(0, 1, 1);
        mat.set(0, 2, 1);

        mat.set(1, 0, 1);
        mat.set(1, 3, 1);
        mat.set(1, 4, 1);

        mat.set(2, 1, 1);
        mat.set(2, 3, 1);
        mat.set(2, 5, 1);

        mat.set(3, 2, 1);
        mat.set(3, 4, 1);
        mat.set(3, 5, 1);

        reduce_z2(&mut mat, 0);

        // 1 0 0 0 0 0
        // 0 1 0 0 0 0
        // 0 0 1 0 0 0
        // 0 0 0 0 0 0

        // TODO: Implement Eq/PartialEq for Matrix?

        assert_eq!(*mat.get(0, 0).unwrap() % 2, 1);
        for i in 1..6 {
            assert_eq!(*mat.get(0, i).unwrap() % 2, 0);
        }

        assert_eq!(*mat.get(0, 1).unwrap() % 2, 0);
        assert_eq!(*mat.get(1, 1).unwrap() % 2, 1);
        for i in 2..6 {
            assert_eq!(*mat.get(1, i).unwrap() % 2, 0);
        }

        for i in 0..2 {
            assert_eq!(*mat.get(2, i).unwrap() % 2, 0);
        }
        assert_eq!(*mat.get(2, 2).unwrap() % 2, 1);
        for i in 3..6 {
            assert_eq!(*mat.get(2, i).unwrap() % 2, 0);
        }

        for i in 0..6 {
            assert_eq!(*mat.get(3, i).unwrap() % 2, 0);
        }
    }

    #[test]
    fn test_reduce_case_2() {
        let mut mat = DenseMatrix::<usize>::new(6, 4);

        // https://courses.cs.duke.edu//fall06/cps296.1/Lectures/sec-IV-3.pdf
        // 1 1 0 0
        // 1 0 1 0
        // 0 1 1 0
        // 1 0 0 1
        // 0 1 0 1
        // 0 0 1 1

        mat.set(0, 0, 1);
        mat.set(0, 1, 1);

        mat.set(1, 0, 1);
        mat.set(1, 2, 1);

        mat.set(2, 1, 1);
        mat.set(2, 2, 1);

        mat.set(3, 0, 1);
        mat.set(3, 3, 1);

        mat.set(4, 1, 1);
        mat.set(4, 3, 1);

        mat.set(5, 2, 1);
        mat.set(5, 3, 1);

        reduce_z2(&mut mat, 0);

        // 1 0 0 0
        // 0 1 0 0
        // 0 0 1 0
        // 0 0 0 0
        // 0 0 0 0
        // 0 0 0 0

        assert_eq!(*mat.get(0, 0).unwrap() % 2, 1);
        assert_eq!(*mat.get(1, 1).unwrap() % 2, 1);
        assert_eq!(*mat.get(2, 2).unwrap() % 2, 1);

        for i in 3..6 {
            for j in 0..4 {
                assert_eq!(*mat.get(i, j).unwrap() % 2, 0);
            }
        }
    }
}
