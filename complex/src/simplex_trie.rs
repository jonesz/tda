// src/complex/src/simplex_trie.rs
use crate::simplex::Simplex;

pub trait SimplexTrie {
    fn new() -> Self;
    fn new_skel(sz: usize) -> Self;
    fn add_simplex(&mut self, simplex: &Simplex);
    fn contains_simplex(&self, simplex: &Simplex) -> bool;
}

pub trait IntoSimplexDimIter {
    type SimplexDimIter;
    fn iter_dim(self, sz: usize) -> Self::SimplexDimIter;
}
