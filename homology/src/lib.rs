// homology/src/lib.rs
//! Computing homology of a SimplicialComplex.
mod boundary;
mod reduction;

use common::Matrix;
use complex::simplex::Simplex;
use complex::simplex_trie::{IntoSimplexDimIter, SimplexTrie};

/// Compute the 'p'th betti number of a SimplicialComplex.
pub fn compute_homology<S: SimplexTrie + IntoSimplexDimIter<Item = Simplex>>(
    complex: &S,
    p: usize,
) -> usize {
    // TODO: Handle the 0'th betti number case, which is an edge case.

    // Zp
    let mut zmat = boundary::boundary(complex, p);
    reduction::reduce_z2(&mut zmat, 0);

    // Bp
    let mut bmat = boundary::boundary(complex, p + 1);
    reduction::reduce_z2(&mut bmat, 0);

    let (_, z_cols) = zmat.dim();
    let (b_rows, _) = bmat.dim();

    let mut rank_z: usize = 0;
    let mut rank_b: usize = 0;

    // Zp: the number of zero columns.
    for i in 0..z_cols {
        if let Some(v) = zmat.get(i, i) {
            match v % 2 {
                0 => rank_z += 1,
                1 => (),
                _ => unreachable!(),
            }
        }
    }

    // Bp: the number of non-zero columns.
    for i in 0..b_rows {
        if let Some(v) = zmat.get(i, i) {
            match v % 2 {
                0 => (),
                1 => rank_b += 1,
                _ => unreachable!(),
            }
        }
    }

    rank_z - rank_b
}
