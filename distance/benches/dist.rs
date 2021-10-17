// src/distance/benches/dist.rs
#![feature(test)]
extern crate test;
use distance::{Metric, MetricFn};
use rand::prelude::*;
use test::Bencher;

#[bench]
fn bench_euclidean(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let x: Vec<f64> = (0..1000).map(|_| rng.gen()).collect();
    let y: Vec<f64> = (0..1000).map(|_| rng.gen()).collect();

    b.iter(|| x.dist(&y, Some(MetricFn::Euclidean)))
}

#[bench]
fn bench_manhattan(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let x: Vec<f64> = (0..1000).map(|_| rng.gen()).collect();
    let y: Vec<f64> = (0..1000).map(|_| rng.gen()).collect();

    b.iter(|| x.dist(&y, Some(MetricFn::Manhattan)))
}
