use crate::SimplicialComplex;
use std::borrow::Cow;
use std::io::Write;

type Nd = usize;
type Ed = (usize, usize);

pub fn render_to<W: Write>(complex: SimplicialComplex, output: &mut W) {
    dot::render(&complex, output).unwrap()
}

impl<'a> dot::Labeller<'a, Nd, Ed> for SimplicialComplex {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("SimplicialComplex").unwrap()
    }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", *n)).unwrap()
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed> for SimplicialComplex {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        (0..self.simplices[0].len()).collect()
    }

    // The edges of the graph are captured within the 1-simplex structure.
    fn edges(&self) -> dot::Edges<'a, Ed> {
        // TODO: Refactor this to use map.
        let mut p = vec![];
        for i in 0..self.simplices[1].len() {
            let simplex = &self.simplices[1][i];
            p.push((simplex.vertices[0], simplex.vertices[1]))
        }

        Cow::from(p)
    }

    fn source(&self, e: &Ed) -> Nd {
        e.0
    }
    fn target(&self, e: &Ed) -> Nd {
        e.1
    }
}
