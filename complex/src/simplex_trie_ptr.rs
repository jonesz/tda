// src/complex/src/simplex_trie.rs
//! Referenced from: 'The Simplex Tree: An Efficient Data Structure for
//! General Simplicial Complexes - Jean-Daniel Boissonnat · Clément Maria'
use crate::simplex::{Simplex, Vertex};
use std::collections::{BTreeMap, VecDeque};

// We're attempting to produce a trie structure; as an example:
//   1     2   3   4
//  2 3 4      4
// 3
// Contains the simplices: (1, 2, 3), (1, 2), (1, 3), (1), (2), ... etc.

struct Node<T>(BTreeMap<T, Node<T>>);

impl<T> Node<T>
where
    T: Ord + Copy,
{
    pub fn new() -> Self {
        Node(BTreeMap::new())
    }

    /// Add a value to the trie.
    pub fn add(&mut self, value: &[T]) {
        if let Some((x, xs)) = value.split_first() {
            if !self.0.contains_key(x) {
                self.0.insert(*x, Node::new());
            }

            self.0.get_mut(x).unwrap().add(xs);
        }
    }

    /// Determine whether a value is contained within the trie.
    pub fn contains(&self, value: &[T]) -> bool {
        if let Some((x, xs)) = value.split_first() {
            match self.0.get(x) {
                Some(v) => v.contains(xs),
                None => false,
            }
        } else {
            true
        }
    }
}

/// A SimplexTrie contains a single root node, corresponding to the empty
/// set; simplices contained within K begin as child nodes off that root node.
pub struct SimplexTrie(Node<Vertex>);

impl Default for SimplexTrie {
    fn default() -> Self {
        Self::new()
    }
}

impl SimplexTrie {
    /// Create a new, empty SimplexTrie
    pub fn new() -> Self {
        SimplexTrie(Node::new())
    }

    /// Create a new SimplexTrie with a 0-skeleton of size 'sz'.
    pub fn new_skel(sz: usize) -> Self {
        let mut st = SimplexTrie(Node::new());
        for i in 0..sz {
            st.0.add(&[Vertex::new(i, 0); 1])
        }

        st
    }

    // TODO: Rather than exposing a public function '.vertices()' on Simplex,
    // is there a trait for 'to_slice()' ? Or utilize the fact we've already
    // implemented iterators for Simplex.
    pub fn add_simplex(&mut self, simplex: &Simplex) {
        self.0.add(simplex.vertices())
    }

    // TODO: See above note in 'add_simplex'.
    pub fn contains_simplex(&self, simplex: &Simplex) -> bool {
        self.0.contains(simplex.vertices())
    }

    /// Iterate, returning simplices of dimension 'sz'.
    pub fn iter_dim(&self, sz: usize) -> SimplexTrieIterator {
        let mut iter = self.into_iter();
        iter.1 = Some(sz);
        iter
    }
}

type QueueValue<'a> = ((&'a Vertex, &'a Node<Vertex>), bool);

/// Queue, Option<Dimension>.
pub struct SimplexTrieIterator<'a>(VecDeque<QueueValue<'a>>, Option<usize>);

impl<'a> Iterator for SimplexTrieIterator<'a> {
    type Item = Simplex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.pop_front() {
            None => None,
            Some(x) => {
                let mut head = vec![x];

                // Pop values off until we find another head value or exhaust
                // the queue.
                while let Some(y) = self.0.front() {
                    match y.1 {
                        true => break,
                        false => {
                            // Encountered a child value, pop off the queue
                            // and push to the head.
                            head.push(self.0.pop_front().unwrap());
                        }
                    }
                }

                let smplx = Simplex::new(head.iter().map(|a| *a.0 .0).collect());

                // If the last value of head has children, we need to push
                // them onto the stack if:
                // A) there is no dimensional constraint on the iterator.
                //  OR
                // B) the dimension of the above simplex 'smplx' is smaller
                // than the dimensional constraint.

                // TODO: Could be written in such a way that the for loop
                // is written only once; will the compiler hoist out the
                // dimension check?
                match self.1 {
                    Some(dim) => match dim > smplx.dim() {
                        true => {
                            for child in head.last().unwrap().0 .1 .0.iter() {
                                let mut smplx = head.clone();
                                smplx.push((child, false));
                                self.0.extend(smplx);
                            }
                        }
                        false => (),
                    },

                    None => {
                        for child in head.last().unwrap().0 .1 .0.iter() {
                            let mut smplx = head.clone();
                            smplx.push((child, false));
                            self.0.extend(smplx);
                        }
                    }
                }

                // Return the smplx if it matches our dimensional constraint.
                match self.1 {
                    Some(dim) => match dim == smplx.dim() {
                        true => Some(smplx),
                        false => self.next(),
                    },
                    None => Some(smplx),
                }
            }
        }
    }
}

impl<'a> IntoIterator for &'a SimplexTrie {
    type Item = Simplex;
    type IntoIter = SimplexTrieIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = SimplexTrieIterator(VecDeque::new(), None);

        // push the 0-skeleton onto the stack.
        for node in &self.0 .0 {
            iter.0.push_back((node, true))
        }

        iter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_add_contains() {
        let mut root = Node::<Vertex>::new();

        let simplex = Simplex::new(vec![
            Vertex::new(1, 0),
            Vertex::new(2, 0),
            Vertex::new(3, 4),
        ]);
        root.add(simplex.vertices());
        assert_eq!(root.contains(simplex.vertices()), true);

        // Should return false.
        let simplex = Simplex::new(vec![Vertex::new(2, 0)]);
        assert_eq!(root.contains(simplex.vertices()), false);
    }

    #[test]
    fn test_simplex_trie_iter() {
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
