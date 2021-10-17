// src/complex/src/simplex_trie_cyclic.rs
//! Referenced from: 'The Simplex Tree: An Efficient Data Structure for
//! General Simplicial Complexes - Jean-Daniel Boissonnat · Clément Maria'

struct Node<T> {
    children: BTreeMap<T, Node<T>>,
    parent: Option<Node<T>>,
}

impl<T> Node<T>
where
    T: Ord + Copy,
{
    pub fn new(parent: Option<Node<T>>) -> Self {
    }
}

struct SimplexTrie {
    root: Node<Vertex>,
}
