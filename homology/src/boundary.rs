// homology/src/boundary.rs
//! Producing boundary matrices of a SimplicialComplex.
use common::dense::DenseMatrix;
use common::Matrix;
use complex::simplex::Simplex;
use complex::simplex_trie::{IntoSimplexDimIter, SimplexTrie};

/// Return the boundary matrix for the p'th dimension.
pub fn boundary<S: SimplexTrie + IntoSimplexDimIter<Item = Simplex>>(
    complex: &S,
    p: usize,
) -> DenseMatrix<usize> {
    // TODO: The zero'th betti number is the number of connected components,
    // which is the number of edges in the graph?
    if p == 0 {
        panic!("Shouldn't be called for 0'th betti number.")
    }

    // TODO: Count the number of simplices of each dimension during the
    // creation of the trie or offer up a function that quickly computes
    // the number of values at a certain depth.
    let rows = complex.iter_dim(p - 1).count();
    let cols = complex.iter_dim(p).count();

    // Rows are the number of p-1 simplices; cols p-simplices.
    let mut mat = DenseMatrix::<usize>::new(rows, cols);

    for (j, bigger) in complex.iter_dim(p).enumerate() {
        for (i, smaller) in complex.iter_dim(p - 1).enumerate() {
            if smaller.is_face(&bigger) {
                mat.set(i, j, 1);
            }
        }
    }

    mat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary() {
        panic!()
    }
}
