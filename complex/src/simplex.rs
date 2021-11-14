// src/complex/src/simplex.rs
//! Utilities for interacting with Simplices.
use std::cmp::Ordering;
use std::fmt;
use std::slice;

/// A Vertex consists of an id and a weight.
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vertex(usize, usize);

impl Vertex {
    pub fn new(id: usize, weight: usize) -> Self {
        Vertex(id, weight)
    }

    pub fn id(&self) -> usize {
        self.0
    }

    pub fn weight(&self) -> usize {
        self.1
    }
}

impl Ord for Vertex {
    /// Compare two vertices; first by weight then id.
    fn cmp(&self, other: &Self) -> Ordering {
        let r = self.1.cmp(&other.1);
        match r {
            Ordering::Equal => self.0.cmp(&other.0),
            _ => r,
        }
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Vertex {}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.0, self.1)
    }
}

/// A Simplex consists of a set of Vertices.
#[derive(Debug, PartialEq, Clone)]
pub struct Simplex(Vec<Vertex>);

impl Simplex {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Simplex(vertices)
    }

    /// Return the weight of a Simplex, which is the last added edge.
    pub fn weight(&self) -> usize {
        match self.0.iter().reduce(|a, b| if a.1 > b.1 { a } else { b }) {
            Some(vertex) => vertex.1,
            None => panic!("Encountered a Simplex with no vertices."),
        }
    }

    /// Return the dimension of a Simplex.
    pub fn dim(&self) -> usize {
        // TODO: Triggers an underflow on an invalid Simplex; is killing
        // the program if we encounter one alright?
        self.0.len() - 1
    }

    // TODO: See note within `simplex_trie.rs` about the removal of this
    // function.
    pub fn vertices(&self) -> &[Vertex] {
        &self.0
    }

    /// Return whether this simplex is a face of another simplex.
    pub fn is_face(&self, other: &Self) -> bool {
        // Avoid underflow.
        match other.dim() {
            0 => return false,
            _ => (),
        }

        match self.dim() == other.dim() - 1 {
            true => {
                // Each vertex should be a vertex on the other simplex.
                for vrtx in self {
                    if !other.0.contains(vrtx) {
                        return false;
                    }
                }
                true
            }
            false => false,
        }
    }
}

impl Ord for Simplex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}

impl PartialOrd for Simplex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Simplex {}

impl fmt::Display for Simplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Simplex(")?;
        for vertex in &self.0 {
            write!(f, "{}, ", vertex)?;
        }

        write!(f, "weight: {})", self.weight())
    }
}

impl IntoIterator for Simplex {
    type Item = Vertex;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Simplex {
    type Item = &'a Vertex;
    type IntoIter = slice::Iter<'a, Vertex>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_fmt() {
        let vertex = Vertex(1, 2);
        assert_eq!(format!("{}", vertex), "(1, 2)");
    }

    #[test]
    fn test_simplex_weight() {
        let simplex = Simplex(vec![Vertex(1, 5), Vertex(2, 3), Vertex(5, 1)]);
        assert_eq!(simplex.weight(), 5);
    }

    #[test]
    fn test_simplex_dimension() {
        let simplex = Simplex(vec![Vertex(1, 5), Vertex(2, 3), Vertex(5, 1)]);
        assert_eq!(simplex.dim(), 2);
    }

    #[test]
    fn test_simplex_iterator() {
        let simplex = Simplex(vec![Vertex(1, 0), Vertex(2, 3), Vertex(5, 1)]);
        let mut iter = simplex.into_iter();
        assert_eq!(iter.next(), Some(Vertex(1, 0)));
        assert_eq!(iter.next(), Some(Vertex(2, 3)));
        assert_eq!(iter.next(), Some(Vertex(5, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_simplex_ref_iterator() {
        let simplex = &Simplex(vec![Vertex(1, 0), Vertex(2, 3), Vertex(5, 1)]);
        let mut iter = simplex.into_iter();
        assert_eq!(iter.next(), Some(&Vertex(1, 0)));
        assert_eq!(iter.next(), Some(&Vertex(2, 3)));
        assert_eq!(iter.next(), Some(&Vertex(5, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_simplex_fmt() {
        let simplex = Simplex(vec![Vertex(1, 0), Vertex(2, 1), Vertex(5, 2)]);
        assert_eq!(
            format!("{}", simplex),
            "Simplex((1, 0), (2, 1), (5, 2), weight: 2)"
        );
    }

    #[test]
    fn test_simplex_is_face() {
        // 0-skeleton case.
        let simplex_a = Simplex(vec![Vertex(1, 0)]);
        let simplex_b = Simplex(vec![Vertex(2, 0)]);
        assert_eq!(simplex_a.is_face(&simplex_b), false);

        // 1-skeleton false case.
        let simplex_a = Simplex(vec![Vertex(1, 0)]);
        let simplex_b = Simplex(vec![Vertex(2, 0), Vertex(3, 0)]);
        assert_eq!(simplex_a.is_face(&simplex_b), false);

        // 1-skeleton true case.
        let simplex_a = Simplex(vec![Vertex(1, 0)]);
        let simplex_b = Simplex(vec![Vertex(1, 0), Vertex(3, 0)]);
        assert_eq!(simplex_a.is_face(&simplex_b), true);
    }
}
