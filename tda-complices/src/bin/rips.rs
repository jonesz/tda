use distance::{dist, Metric};
use rand::prelude::*;
use tda_complices::rips::naive;
use tda_complices::viz::graphviz;

fn sample(k: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    let mut res = vec![];
    for _ in 0..k {
        res.push(vec![rng.gen()])
    }

    res
}

fn main() {
    let rips = naive::rips(dist(sample(30), Metric::Manhattan), 0.01, 4);
    use std::fs::File;
    let mut f = File::create("example.dot").unwrap();
    println!("{:?}", rips);
    graphviz::render_to(rips, &mut f);
}
