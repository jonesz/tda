pub mod rips;
pub mod viz;

#[derive(Debug)]
pub struct Simplex {
    vertices: Vec<usize>,
}

#[derive(Debug)]
pub struct SimplicialComplex {
    simplices: Vec<Vec<Simplex>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
