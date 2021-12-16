// src/complex/src/simplex_trie.rs
use crate::simplex::Simplex;

pub trait SimplexTrie {
    type SimplexTrieIterator;

    fn new() -> Self;
    fn new_skel(sz: usize) -> Self;
    fn add_simplex(&mut self, simplex: &Simplex);
    fn contains_simplex(&mut self, simpleX: &Simplex) -> bool;
    fn iter_dim(&self, sz: usize) -> Self::SimplexTrieIterator;
}
