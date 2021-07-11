// src/rips/naive.rs
use crate::{Simplex, SimplicialComplex};
use std::collections::HashSet;
use std::iter::FromIterator;

// This specific implementation is built from the paper "Fast Construction of
// the Vietoris-Rips Complex - Afra Zomordian".

/// Compute the neighborhood graph for a distance matrix.
fn compute_neighborhood(dist_mat: Vec<Vec<f64>>, epsilon: f64) -> Vec<Vec<u8>> {
    // TODO: This is obviously O(nlogn); compute the neighborhood of
    // combinations. All-pairs bruteforce: O(n^2).
    dist_mat
        .iter()
        .enumerate()
        .map(|(j, x)| {
            x.iter()
                .enumerate()
                .map(|(k, y)| if *y < epsilon && j != k { 1 } else { 0 })
                .collect()
        })
        .collect()
}

/// Compute the neighborhood graph for a distance matrix non-deterministically.
fn compute_neighborhood_nondeterministic(dist_mat: Vec<Vec<f64>>, epsilon: f64) -> Vec<Vec<u8>> {
    panic!();
}

/// Finds all neighbors of a vertex u within the neighborhood that precede
/// it in the given ordering.
fn lower_nbrs(neighborhood: &[Vec<u8>], u: &usize) -> Vec<usize> {
    // TODO: There is a 'filter_map' within iterator; using that might
    // improve perf?
    neighborhood[*u]
        .iter()
        .enumerate()
        .filter(|(v, x)| (u > v) && (**x == 1u8))
        .map(|(v, _)| v)
        .collect()
}

/// The inductive algorithm presented.
fn inductive(neighborhood: Vec<Vec<u8>>, k: usize) -> SimplicialComplex {
    let mut simplices = vec![];
    let mut zero_simplices = vec![];
    for (k, _) in neighborhood.iter().enumerate() {
        zero_simplices.push(Simplex { vertices: vec![k] });
    }
    simplices.push(zero_simplices);

    // The simplicial complex now consists of the zero-skeleton.

    for i in 1..k {
        let mut n_simplices = vec![];
        for tau in &simplices[i - 1] {
            let mut set: HashSet<usize> = HashSet::new();
            for (i, u) in tau.vertices.iter().enumerate() {
                let n = lower_nbrs(&neighborhood, u);
                if i == 0 {
                    set = HashSet::from_iter(n);
                } else {
                    let tmp = HashSet::from_iter(n);
                    set = set.intersection(&tmp).copied().collect();
                }
            }

            for element in set {
                let mut vertices = tau.vertices.clone();
                vertices.push(element);
                n_simplices.push(Simplex { vertices });
            }
        }

        simplices.push(n_simplices);
    }

    SimplicialComplex { simplices }
}

fn incremental(mat: Vec<Vec<f64>>, epsilon: f64) -> SimplicialComplex {
    panic!();
}

pub fn rips(mat: Vec<Vec<f64>>, epsilon: f64, k: usize) -> SimplicialComplex {
    inductive(compute_neighborhood(mat, epsilon), k)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compute_neighborhood_exact() {
        let a = vec![0.0, 3.0];
        let b = vec![3.0, 0.0];
        let dist_mat = vec![a.as_slice(), b.as_slice()];

        let res = compute_neighborhood(&dist_mat, 1.0);
        assert_eq!(res, vec![vec![0, 0], vec![0, 0]]);
        let res = compute_neighborhood(&dist_mat, 6.0);
        assert_eq!(res, vec![vec![0, 1], vec![1, 0]]);
    }
}
