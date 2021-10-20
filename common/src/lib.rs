// src/common/src/lib.rs
//! Matrix operations.
pub mod dense;
pub mod sparse;

/// A trait with basic operations for an `m x n` Matrix.
pub trait Matrix<T> {
    fn new(r: usize, c: usize) -> Self;

    /// Return the (rows, columns) of a Matrix.
    fn dim(&self) -> (usize, usize);
    fn set(&mut self, r: usize, c: usize, value: T);
    fn get(&self, r: usize, c: usize) -> Option<&T>;
}

// TODO: The trait requirements should be Add + Matrix, but compiler was
// complaining about an implementation I thought I already had.
pub trait MatrixOps<T>
where
    T: std::ops::Add,
{
    fn col_swap(&mut self, x: usize, y: usize);
    fn row_swap(&mut self, x: usize, y: usize);

    /// Add col 'x' to col 'y'.
    fn col_add(&mut self, x: usize, y: usize);
    /// Add row 'x' to col 'y'.
    fn row_add(&mut self, x: usize, y: usize);
}
