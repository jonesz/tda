// src/complex/src/simplex_trie_arena.rs
//! Referenced from: 'The Simplex Tree: An Efficient Data Structure for
//! General Simplicial Complexes - Jean-Daniel Boissonnat · Clément Maria'
use crate::simplex::{Simplex, Vertex};

/// A trie backed with an arena.
struct TrieArena<T> {
    arena: Vec<T>,
    // Each value within the arena can have multiple children.
    children: Vec<Vec<usize>>,
    // Each value within the arena has a single parent.
    parent: Vec<usize>,
    // A listing of every node that resides at a certain depth.
    depth: Vec<Vec<usize>>,
}

impl<T> TrieArena<T>
where
    T: PartialEq + Copy + Default,
{
    pub fn new() -> Self {
        let mut t = TrieArena {
            arena: Vec::new(),
            children: Vec::new(),
            parent: Vec::new(),
            depth: Vec::new(),
        };

        // Place the root at position zero.
        t.arena.push(T::default());

        // The root has no parent.
        t.parent.push(0);

        // Push a vector for the children of the root.
        t.children.push(Vec::new());

        t.depth.push(Vec::new());

        t
    }

    fn _add(&mut self, value: &[T], parent: usize, depth: usize) {
        if let Some((x, xs)) = value.split_first() {
            // Attempt to find the value x within the child values of
            // our parent.
            for child in self.children.get(parent).unwrap() {
                if self.arena.get(*child).unwrap() == x {
                    let new_parent = *child;
                    return self._add(xs, new_parent, depth + 1);
                }
            }

            // Unable to find x; add x to the trie.
            self.arena.push(*x);
            self.children.push(Vec::new());
            self.parent.push(parent);

            let x_index = self.arena.len() - 1;

            // The current parent has this index as a child now.
            self.children.get_mut(parent).unwrap().push(x_index);

            if self.depth.len() <= depth {
                self.depth.push(Vec::new());
            }

            // x resides in the current depth.
            self.depth.get_mut(depth).unwrap().push(x_index);

            return self._add(xs, x_index, depth + 1);
        }
    }

    /// Add a value to the trie.
    pub fn add(&mut self, value: &[T]) {
        self._add(value, 0, 0)
    }

    fn _contains(&self, value: &[T], parent: usize) -> bool {
        if let Some((x, xs)) = value.split_first() {
            for child in self.children.get(parent).unwrap() {
                if self.arena.get(*child).unwrap() == x {
                    let new_parent = *child;
                    return self._contains(xs, new_parent);
                }
            }
            false
        } else {
            true
        }
    }

    pub fn contains(&self, value: &[T]) -> bool {
        self._contains(value, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triearena_add_contains() {
        let mut trie: TrieArena<Vertex> = TrieArena::new();

        // 0-degree simplex at the root.
        let simplex = Simplex::new(vec![Vertex::new(1, 0)]);
        trie.add(simplex.vertices());
        assert_eq!(trie.contains(simplex.vertices()), true);

        // 0-degree simplex not contained within the trie.
        let simplex = Simplex::new(vec![Vertex::new(2, 0)]);
        assert_eq!(trie.contains(simplex.vertices()), false);

        // 2-degree simplex.
        let simplex = Simplex::new(vec![
            Vertex::new(1, 0),
            Vertex::new(2, 1),
            Vertex::new(3, 1),
        ]);
        trie.add(simplex.vertices());
        assert_eq!(trie.contains(simplex.vertices()), true);

        // 1-degree simplex contained within the above 3-degree simplex.
        let simplex = Simplex::new(vec![Vertex::new(1, 0), Vertex::new(2, 1)]);
        assert_eq!(trie.contains(simplex.vertices()), true);
    }
}
