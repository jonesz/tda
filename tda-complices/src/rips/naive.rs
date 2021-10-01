// src/rips/naive.rs
use crate::{Filtration, Simplex, SimplicialComplex};
use common::Matrix;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

// This specific implementation is built from the paper "Fast Construction of
// the Vietoris-Rips Complex - Afra Zomordian".

/// Compute the neighborhood graph for a distance matrix.
fn compute_neighborhood(mat: &Matrix<f64>, epsilon: f64) -> (Matrix<bool>, f64) {
    let mut out_mat = Matrix::new(mat.cols, mat.rows);
    // Smallest value encountered that is greater than epsilon.
    let mut eps = std::f64::MAX;

    // TODO: This is obviously O(nlogn); compute the neighborhood of
    // combinations. All-pairs bruteforce: O(n^2).
    for i in 0..mat.cols {
        for j in 0..mat.rows {
            let tmp = mat.get(i, j).unwrap();
            match tmp <= epsilon {
                true => out_mat.set(i, j, true),
                false => {
                    out_mat.set(i, j, false);
                    if tmp < eps {
                        eps = tmp;
                    }
                }
            }
        }
    }

    (out_mat, eps)
}

/// Finds all neighbors of a vertex u within the neighborhood that precede
/// it in the given ordering.
fn lower_nbrs(neighborhood: &Matrix<bool>, u: usize) -> Vec<usize> {
    let mut v = vec![];

    for i in 0..neighborhood.rows {
        let vrtx = neighborhood.get(u, i).unwrap();
        if u > i && vrtx {
            v.push(i);
        }
    }

    v
}

/// The inductive algorithm presented.
fn inductive(neighborhood: &Matrix<bool>, k: usize) -> SimplicialComplex {
    // The following is a Vec<Vec> where the outer index contains all Simplices
    // of n-degree.
    let mut simplices = vec![];

    let mut zero_simplices = vec![];
    for k in 0..neighborhood.cols {
        zero_simplices.push(Simplex { vertices: vec![k] });
    }

    // After this operation, the simplicial complex now consists of the
    // zero-skeleton.
    simplices.push(zero_simplices);

    for i in 1..k {
        let mut n_simplices = vec![];
        for tau in &simplices[i - 1] {
            let mut set: HashSet<usize> = HashSet::new();
            for (i, u) in tau.vertices.iter().enumerate() {
                let n = lower_nbrs(&neighborhood, *u);
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

/// Compute the Vietoris-Rips complex for a distance matrix with cutoff distance
/// epsilon up the k'th degree simplex.
pub fn rips(mat: &Matrix<f64>, epsilon: f64, k: usize) -> SimplicialComplex {
    let (a, _) = compute_neighborhood(mat, epsilon);
    inductive(&a, k)
}

pub fn rips_filter(mat: &Matrix<f64>, k: usize) -> Filtration {
    let mut epsilon = 0.0;

    // The value of epsilon a Simplex first appeared at.
    let mut complex_map: HashMap<Simplex, f64> = HashMap::new();
    // All simplices we encounter throughout the filtration.
    let mut complex_set: Vec<HashSet<Simplex>> = Vec::new();
    complex_set.resize(k, HashSet::new());

    while epsilon != std::f64::MAX {
        let (nbr, new_eps) = compute_neighborhood(mat, epsilon);
        let simplicial_complex = inductive(&nbr, k);

        for (deg, simplices) in simplicial_complex.simplices.iter().enumerate() {
            for simplex in simplices {
                // If the simplex doesn't exist within the complex, push it,
                // then add the epsilon value it occured at to the map.
                let set = &mut complex_set[deg];
                if !set.contains(simplex) {
                    set.insert(simplex.clone());
                    complex_map.insert(simplex.clone(), epsilon);
                }
            }
        }

        epsilon = new_eps;
    }

    let mut simplices = vec![];

    for deg in complex_set {
        for simplex in deg {
            let eps = complex_map.get(&simplex).unwrap();
            simplices.push((*eps, simplex))
        }
    }

    Filtration {
        simplices: simplices,
    }
}
