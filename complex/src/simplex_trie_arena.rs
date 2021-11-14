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

pub struct SimplexTrie(TrieArena<Vertex>);

impl Default for SimplexTrie {
    fn default() -> Self {
        Self::new()
    }
}

impl SimplexTrie {
    /// Create a new, empty SimplexTrie.
    pub fn new() -> Self {
        SimplexTrie(TrieArena::new())
    }

    /// Create a new SimplexTrie with a 0-skeleton of size 'sz'.
    fn new_skel(sz: usize) -> Self {
        let mut st = SimplexTrie::new();
        for i in 0..sz {
            st.0.add(&[Vertex::new(i, 0); 1]);
        }

        st
    }

    pub fn add_simplex(&mut self, simplex: &Simplex) {
        self.0.add(simplex.vertices());
    }

    pub fn contains_simplex(&self, simplex: &Simplex) -> bool {
        self.0.contains(simplex.vertices())
    }

    pub fn iter_dim(&self, sz: usize) -> SimplexTrieIterator {
        let mut iter = self.into_iter();
        iter.0 = sz;
        iter.3 = true;
        iter
    }
}

// Depth, Index, TrieArena.
pub struct SimplexTrieIterator<'a>(usize, usize, &'a TrieArena<Vertex>, bool);

impl<'a> Iterator for SimplexTrieIterator<'a> {
    type Item = Simplex;

    fn next(&mut self) -> Option<Self::Item> {
        // If there's nothing at this depth, the iterator is exhausted.
        if let Some(depth) = self.2.depth.get(self.0) {
            // If we can fetch something at 'index', then we can build a
            // simplex; otherwise, go to the next depth.
            if let Some(v) = depth.get(self.1) {
                let mut index = *v;
                let mut vertices = vec![];
                loop {
                    // If we've made it to the root return the simplex.
                    if index == 0 {
                        self.1 = self.1 + 1;
                        vertices.reverse();
                        return Some(Simplex::new(vertices));
                    } else {
                        vertices.push(*self.2.arena.get(index).unwrap());
                        index = *self.2.parent.get(index).unwrap();
                    }
                }
            } else {
                // If it's a dimensional iter, we're done.
                if self.3 {
                    return None;
                }

                self.0 = self.0 + 1;
                self.1 = 0;
                return self.next();
            }
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a SimplexTrie {
    type Item = Simplex;
    type IntoIter = SimplexTrieIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SimplexTrieIterator(0, 0, &self.0, false)
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

    #[test]
    fn test_simplex_triearena_iter() {
        let mut st = SimplexTrie::new_skel(10);
        let smplx_1 = Simplex::new(vec![Vertex::new(1, 0), Vertex::new(2, 3)]);
        st.add_simplex(&smplx_1);

        // 11-simplices (10 0-skel, 1 1-skel).
        let mut st_iter = st.into_iter();
        for _ in 0..11 {
            assert_eq!(st_iter.next().is_some(), true);
        }

        assert_eq!(st_iter.next().is_some(), false);
    }

    #[test]
    fn test_simplex_trie_iter_dim_0_skel() {
        let st = SimplexTrie::new_skel(10);
        let mut st_iter = st.iter_dim(0);

        // Iteration on the dimension 0, should return 10 entries.
        for i in 0..10 {
            let value = st_iter.next();
            assert_eq!(value.is_some(), true);
            assert_eq!(value.unwrap(), Simplex::new(vec![Vertex::new(i, 0)]));
        }

        // No more simplices.
        assert_eq!(st_iter.next().is_none(), true);
    }

    #[test]
    fn test_simplex_trie_iter_dim_1_skel() {
        let mut st = SimplexTrie::new_skel(10);

        let smplx_1 = Simplex::new(vec![Vertex::new(1, 0), Vertex::new(2, 3)]);
        st.add_simplex(&smplx_1);
        let smplx_2 = Simplex::new(vec![
            Vertex::new(1, 0),
            Vertex::new(2, 3),
            Vertex::new(3, 4),
        ]);
        st.add_simplex(&smplx_2);

        let mut st_iter = st.iter_dim(1);

        // There should be a single 1-simplex, (1, 2).
        let value = st_iter.next();
        assert_eq!(value.is_some(), true);
        assert_eq!(value.unwrap(), smplx_1);

        // No more simplices should be returned.
        assert_eq!(st_iter.next().is_none(), true);
    }
}
