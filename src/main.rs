// src/main.rs
use std::error::Error;
use std::fs::File;
use std::io;
use std::str::FromStr;

use complex::simplex_trie::{SimplexTrie, IntoSimplexDimIter};
use complex::simplex_trie_arena::SimplexTrieArena;
use complex::vietoris_rips;
use distance::{neighborhood, point_cloud};
use viz::graphviz;

fn read() -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut ov = vec![];

    for result in rdr.records() {
        let mut tmp = vec![];
        let record = result?;
        for dim in &record {
            let val: f64 = f64::from_str(dim)?;
            tmp.push(val);
        }

        ov.push(tmp);
    }

    Ok(ov)
}

fn main() {
    match read() {
        Ok(r) => {
            let adj = neighborhood::to_adjacency(&point_cloud::to_dist_mat(&r, None), 0.5);
            let mut sc = SimplexTrieArena::new();
            vietoris_rips::VietorisRips::compute(None, &mut sc, &adj, 3, 0);
            if let Ok(mut v) = File::create("example.dot") {
                graphviz::render_to(&sc, &mut v);
            }
        }

        Err(err) => println!("Error: {}", err),
    }
}
