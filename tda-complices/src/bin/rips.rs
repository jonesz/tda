use distance::{dist, Metric};
use rand::prelude::*;
use tda_complices::rips::naive;

fn sample(k: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    let mut res = vec![];
    for _ in 0..k {
        res.push(vec![rng.gen()])
    }

    res
}

fn main() {
    let rips = naive::rips(dist(sample(3), Metric::Manhattan), 0.3, 2);
    println!("{:?}", rips);
}
