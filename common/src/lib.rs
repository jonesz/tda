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
