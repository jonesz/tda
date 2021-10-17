// src/complex/benches/vr.rs
//! Benchmarking various Vietoris-Rips algorithms.
#![feature(test)]
extern crate test;
use common::{sparse::SparseMatrix, Matrix};
use complex::vietoris_rips::{VRAlgorithm, VietorisRips};
use test::Bencher;

// Produce an adjacency matrix suitable for testing.
fn adj() -> SparseMatrix<bool> {
    let mut sm = SparseMatrix::new(500, 500);
    for c in 0..500 {
        for r in 0..500 {
            if (c * r) + 17 % 2 == 0 {
                sm.set(c, r, true)
            }
        }
    }

    sm
}

#[bench]
fn bench_inductive(b: &mut Bencher) {
    let adj_mat = adj();
    b.iter(|| VietorisRips::compute(Some(VRAlgorithm::Inductive), &adj_mat, 5, 0))
}
