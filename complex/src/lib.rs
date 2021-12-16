// src/complex/src/lib.rs
//! Fundamental utilities for computing and interacting with Simplices.
mod simplex;
pub mod simplex_trie;
mod simplex_trie_arena;
mod simplex_trie_ptr;
pub mod vietoris_rips;

pub enum SimplicialComplexBacking {
    SimplexTrieArena,
    SimplexTriePtr,
}
