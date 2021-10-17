This file details the performance issues with the current implementation of
the iterator for a `SimplexTrie`.

Consider a simplex trie that looks like the following:
    1 2 3 4 5
   2 3    5
  3

Say we want to iterate over all simplices with dimension 2; the current
implementation requires us to build simplices of dimension 0, then dimension 1.

A solution is to maintain a collection of pointers to all values at a depth N;
those values can be iterated over, following a "parent" pointer upward for
each node on the trie. This parent pointer ends up being moderately difficult
to implement, requiring some combination of Rc/RefCell/Weak to avoid the
cyclic relationship the borrow checker doesn't want.

As of now, the inductive algorithm with the VR implementation maintains two
vectors containing k, k+1 during computation.
