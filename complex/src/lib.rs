// src/complex/src/lib.rs
//! Fundamental utilities for computing and interacting with Simplices.
mod simplex;
mod simplex_trie;
mod simplex_trie_arena;
pub mod vietoris_rips;

pub struct SimplicialComplex(simplex_trie::SimplexTrie, usize);

impl SimplicialComplex {
    pub fn iter_dim(&self, sz: usize) -> simplex_trie::SimplexTrieIterator {
        self.0.iter_dim(sz)
    }
}

impl<'a> IntoIterator for &'a SimplicialComplex {
    type Item = simplex::Simplex;
    type IntoIter = simplex_trie::SimplexTrieIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
