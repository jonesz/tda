// viz/src/graphviz.rs
/// Transform a SimplicialComplex into a format suitable for representation
/// via GraphViz.
use complex::SimplicialComplex;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::io::Write;

type Nd = usize;
type Ed = (usize, usize);

#[derive(Debug)]
struct Graph {
    nodes: Vec<Nd>,
    edges: Vec<Ed>,
    weights: HashMap<Ed, usize>,
}

/// Transform a SimplicialComplex into the GraphViz format, writing it to
/// disk.
pub fn render_to<W: Write>(complex: &SimplicialComplex, output: &mut W) {
    let tmp: Graph = complex.into();
    dot::render(&tmp, output).unwrap()
}

impl<'a> dot::Labeller<'a, Nd, Ed> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("example").unwrap()
    }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n)).unwrap()
    }

    fn node_label(&'a self, n: &Nd) -> dot::LabelText<'a> {
        dot::LabelText::LabelStr(format!("{}", n).into())
    }

    fn edge_label(&'a self, e: &Ed) -> dot::LabelText<'a> {
        dot::LabelText::LabelStr(format!("{}", self.weights.get(e).unwrap()).into())
    }
}

// TODO: Read the documentation for Cow.
impl<'a> dot::GraphWalk<'a, Nd, Ed> for Graph {
    fn nodes(&'a self) -> dot::Nodes<'a, Nd> {
        Cow::Borrowed(&self.nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, Ed> {
        Cow::Borrowed(&self.edges)
    }

    fn source(&self, e: &Ed) -> Nd {
        e.0
    }
    fn target(&self, e: &Ed) -> Nd {
        e.1
    }
}

impl From<&SimplicialComplex> for Graph {
    fn from(complex: &SimplicialComplex) -> Graph {
        let mut nodes: HashSet<Nd> = HashSet::new();
        let mut edges: HashSet<Ed> = HashSet::new();
        let mut weights: HashMap<Ed, usize> = HashMap::new();

        // To produce the entirety of the graph, we just need to produce
        // the 1-skeleton of the SimplicialComplex.
        for simplex in complex {
            match simplex.dim() {
                0 => {
                    for vertex in simplex {
                        nodes.insert(vertex.id());
                    }
                }

                1 => {
                    let mut iter = simplex.into_iter();
                    let mut a = iter.next().unwrap();
                    let mut b = iter.next().unwrap();

                    // Order the vertices.
                    match a < b {
                        true => (),
                        false => std::mem::swap(&mut a, &mut b),
                    };

                    edges.insert((a.id(), b.id()));
                    weights.insert((a.id(), b.id()), a.weight());
                }
                _ => continue,
            }
        }

        Graph {
            nodes: Vec::from_iter(nodes),
            edges: Vec::from_iter(edges),
            weights,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_simplicial_complex_to_graph() {
        panic!("Write this test!")
    }
}
