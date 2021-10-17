// src/complex/src/vietoris_rips.rs
//! Utilities for computing a Vietoris-Rips complex.
//! This specific implementation is built from the paper "Fast Construction
//! of the Vietoris-Rips Complex - Afria Zomorodian".
use crate::simplex::{Simplex, Vertex};
use crate::simplex_trie::SimplexTrie;
use crate::SimplicialComplex;
use common::Matrix;
use std::collections::HashSet;

pub enum VRAlgorithm {
    Inductive,
}

/// Find all neighbors of a vertex within the adjacency neighborhood.
fn lower_nbrs<M: Matrix<bool>>(adj: &M, vertex: &Vertex) -> Vec<usize> {
    let mut r = Vec::new();
    let (_, cols) = adj.dim();

    for i in 0..cols {
        if let Some(val) = adj.get(vertex.id(), i) {
            if *val && vertex.id() > i {
                r.push(i);
            }
        }
    }

    r
}

// TODO: Investigate whether the HashSets can be more efficiently utilized in
// this function; I have a feeling there is a lot of copying, re-initializing,
// etc.
/// Compute a Vietoris-Rips complex up to dimension 'dim' with an inductive
/// algorithm.
fn inductive<M: Matrix<bool>>(adj: &M, dim: usize, weight: usize) -> SimplicialComplex {
    let mut sc = SimplicialComplex(SimplexTrie::new());

    // See `ITERATOR_ISSUE.md` for concerns about utilizing an iterator within
    // this function; utilizing two vectors is likely more performant.
    let mut k: Vec<Simplex> = vec![];
    let mut k1: Vec<Simplex> = vec![];

    // Compute the 0-skeleton.
    for i in 0..adj.dim().0 {
        let smplx = Simplex::new(vec![Vertex::new(i, 0)]);
        sc.0.add_simplex(&smplx);
        k.push(smplx);
    }

    for _ in 1..dim {
        for simplex in &k {
            // TODO: The borrow checker throws a fit if this isn't initialized
            // here. Is this a bug?
            let mut shared_vertices: HashSet<usize> = HashSet::new();
            for (i, vertex) in simplex.into_iter().enumerate() {
                let ln = lower_nbrs(adj, vertex);
                match i {
                    // 0, shared_vertices is the union.
                    0 => {
                        shared_vertices = HashSet::from_iter(ln);
                    }
                    // 1, shared_vertices is the intersection.
                    _ => {
                        let tmp: HashSet<usize> = HashSet::from_iter(ln);
                        shared_vertices = tmp.intersection(&shared_vertices).copied().collect();
                    }
                }
            }

            // Glue each shared vertex to the current simplex.
            for uv in shared_vertices {
                let mut vertices: Vec<Vertex> = simplex.clone().into_iter().collect();
                vertices.push(Vertex::new(uv, weight));
                let smplx = Simplex::new(vertices);

                sc.0.add_simplex(&smplx);
                k1.push(smplx);
            }
        }

        // k1 becomes k, k1 is cleared.
        k = k1.clone();
        k1.clear();
    }

    sc
}

pub struct VietorisRips;

impl VietorisRips {
    /// Compute a Vietoris-Rips diagram from an adjacency matrix.
    pub fn compute<M: Matrix<bool>>(
        alg: Option<VRAlgorithm>,
        adj: &M,
        dim: usize,
        weight: usize,
    ) -> SimplicialComplex {
        let alg = alg.unwrap_or(VRAlgorithm::Inductive);
        match alg {
            VRAlgorithm::Inductive => inductive(adj, dim, weight),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::dense::DenseMatrix;

    #[test]
    fn test_lower_nbrs() {
        let mut adj: DenseMatrix<bool> = DenseMatrix::new(10, 10);
        // An adjacency matrix with alternating true/false values.
        for i in 0..10 {
            for j in 0..10 {
                adj.set(
                    i,
                    j,
                    match (i + j) % 2 {
                        0 => false,
                        1 => true,
                        _ => unreachable!(),
                    },
                );
            }
        }

        // 0 == false.
        let ln = lower_nbrs(&adj, &Vertex::new(0, 0));
        assert_eq!(ln, vec![]);

        // (9 + even) % 2 == true.
        let ln = lower_nbrs(&adj, &Vertex::new(9, 0));
        assert_eq!(ln, vec![0, 2, 4, 6, 8]);

        // (4 + odd) % 2 == true.
        let ln = lower_nbrs(&adj, &Vertex::new(4, 0));
        assert_eq!(ln, vec![1, 3]);
    }

    #[test]
    fn test_inductive() {
        let mut adj: DenseMatrix<bool> = DenseMatrix::new(4, 4);
        // An adjacency matrix with alternating true/false values.
        for i in 0..4 {
            for j in 0..4 {
                adj.set(
                    i,
                    j,
                    match (i + j) % 2 {
                        0 => false,
                        1 => true,
                        _ => unreachable!(),
                    },
                );
            }
        }

        let complex = inductive(&adj, 3, 0);

        // The 0-skeleton should be within the complex.
        for i in 0..4 {
            let smplx = Simplex::new(vec![Vertex::new(i, 0)]);
            assert_eq!(complex.0.contains_simplex(&smplx), true);
        }

        //   0 1 2 3
        // 0 0 1 0 1
        // 1 1 0 1 0
        // 2 0 1 0 1
        // 3 1 0 1 0
        let smplx = Simplex::new(vec![Vertex::new(1, 0), Vertex::new(0, 0)]);
        assert_eq!(complex.0.contains_simplex(&smplx), true);

        let smplx = Simplex::new(vec![Vertex::new(3, 0), Vertex::new(2, 0)]);
        assert_eq!(complex.0.contains_simplex(&smplx), true);

        let smplx = Simplex::new(vec![Vertex::new(3, 0), Vertex::new(0, 0)]);
        assert_eq!(complex.0.contains_simplex(&smplx), true);
    }

    #[test]
    fn test_inductive_subset() {
        let mut adj: DenseMatrix<bool> = DenseMatrix::new(10, 10);
        // An adjacency matrix with alternating true/false values.
        for i in 0..10 {
            for j in 0..10 {
                adj.set(
                    i,
                    j,
                    match (i * j) % 2 {
                        0 => false,
                        1 => true,
                        _ => unreachable!(),
                    },
                );
            }
        }

        // complex_1 should be a subset of complex_2.
        let complex_1 = inductive(&adj, 4, 0);
        let complex_2 = inductive(&adj, 5, 0);

        for smplx in &complex_1.0 {
            assert_eq!(complex_2.0.contains_simplex(&smplx), true);
        }
    }
}
