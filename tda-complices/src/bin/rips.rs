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
        res.push(vec![rng.gen(), rng.gen()])
    }

    res
}

fn main() {
    let s = sample(10);
    println!("{:?}", s);

    let rips = naive::rips(
        &point_cloud_dist(s, Some(MetricFn::Manhattan), None),
        0.1,
        4,
    );
    let mut f = File::create("example.dot").unwrap();
    println!("{:?}", rips);
    graphviz::render_to(rips, &mut f);

    let s = sample(10);
    let rips_filter = naive::rips_filter(&point_cloud_dist(s, Some(MetricFn::Manhattan), None), 10);

    println!("{:?}", rips_filter);
}
