use distance::{point_cloud_dist, MetricFn};
use rand::prelude::*;

use tda_complices::rips::naive;
use tda_complices::viz::graphviz;

use std::fs::File;

/// Produce a point cloud.
fn sample(k: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    let mut res = vec![];
    for _ in 0..k {
        res.push(vec![rng.gen()])
    }

    res
}

fn main() {
    let rips = naive::rips(
        &point_cloud_dist(&sample(30), Some(MetricFn::Manhattan), None),
        0.1,
        4,
    );
    let mut f = File::create("example.dot").unwrap();
    println!("{:?}", rips);
    graphviz::render_to(rips, &mut f);
}
