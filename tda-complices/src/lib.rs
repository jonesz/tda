// src/tda-complices/src/lib.rs
pub mod rips;
pub mod viz;
pub mod witness;

use std::cmp::PartialEq;

/// A set of connected vertices.
#[derive(Debug, Hash, Eq, Clone)]
pub struct Simplex {
    vertices: Vec<usize>,
}

impl PartialEq for Simplex {
    fn eq(&self, other: &Self) -> bool {
        if self.vertices.len() != other.vertices.len() {
            return false;
        } else {
            for i in 0..self.vertices.len() {
                if self.vertices[i] != other.vertices[i] {
                    return false;
                }
            }
        }

        true
    }
}

/// The set of simplices.
#[derive(Debug)]
pub struct SimplicialComplex {
    simplices: Vec<Vec<Simplex>>,
}

/// A filtration.
#[derive(Debug)]
pub struct Filtration {
    simplices: Vec<(f64, Simplex)>,
}
