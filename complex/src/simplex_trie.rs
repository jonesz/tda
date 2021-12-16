// src/complex/src/simplex_trie.rs
use crate::simplex::Simplex;

pub trait SimplexTrie {
    fn new() -> Self;
    fn new_skel(sz: usize) -> Self;
    fn add_simplex(&mut self, simplex: &Simplex);
    fn contains_simplex(&self, simplex: &Simplex) -> bool;
}

// TODO: Should iter_dim take a reference to self? The items we are returning
// (Simplex) *have* to be constructed; we can't return a reference to them.
// Think through how this actually gets implemented in ASM.
pub trait IntoSimplexDimIter {
    type Item;
    type SimplexDimIter: Iterator<Item = Self::Item>;

    fn iter_dim(&self, sz: usize) -> Self::SimplexDimIter;
}
