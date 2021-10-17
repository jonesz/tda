// src/distance/benches/f64_specialization.rs
#![feature(test)]
extern crate test;
use rand::prelude::*;
use test::Bencher;

fn euclidean_f64(a: &[f64], b: &[f64]) -> f64 {
    let r: f64 = a
        .iter()
        .zip(b.iter())
        .into_iter()
        .fold(0.0, |acc, (x, y)| acc + (x - y).powi(2));
    r.sqrt()
}

fn manhattan_f64(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .into_iter()
        .fold(0.0, |acc, (x, y)| acc + (x - y).abs())
}

#[bench]
fn bench_euclidean(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let x: Vec<f64> = (0..1000).map(|_| rng.gen()).collect();
    let y: Vec<f64> = (0..1000).map(|_| rng.gen()).collect();

    b.iter(|| euclidean_f64(&x, &y))
}

#[bench]
fn bench_manhattan(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let x: Vec<f64> = (0..1000).map(|_| rng.gen()).collect();
    let y: Vec<f64> = (0..1000).map(|_| rng.gen()).collect();

    b.iter(|| manhattan_f64(&x, &y))
}
