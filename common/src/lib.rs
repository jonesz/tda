// src/lib.rs

pub struct Matrix<T: Copy> {
    pub cols: usize,
    pub rows: usize,
    buf: Vec<T>,
}

impl<T: Copy> Matrix<T> {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut m = Matrix {
            cols: cols,
            rows: rows,
            buf: Vec::with_capacity(rows * cols),
        };
        unsafe {
            m.buf.set_len(rows * cols);
        }

        m
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        self.buf[self.rows * i + j] = value;
    }

    pub fn get(&self, i: usize, j: usize) -> Option<T> {
        Some(self.buf[self.rows * i + j])
    }
}
