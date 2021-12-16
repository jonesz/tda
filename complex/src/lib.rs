// src/complex/src/lib.rs
//! Fundamental utilities for computing and interacting with Simplices.
pub mod simplex;
pub mod simplex_trie;
pub mod vietoris_rips;

// TODO: Rather than exposing these publicly, perhaps pass 'Backing' enum and
// receive one back?
pub mod simplex_trie_arena;
pub mod simplex_trie_ptr;

pub enum SimplicialComplexBacking {
    SimplexTrieArena,
    SimplexTriePtr,
}
