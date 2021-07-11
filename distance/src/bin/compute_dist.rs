// src/bin/compute_dist.rs
extern crate csv;

use distance::dist;
use distance::Metric;
use std::env;
use std::error::Error;
use std::fs::File;

fn run() -> Result<(), Box<dyn Error>> {
    // Capture arguments.
    let args: Vec<String> = std::env::args().collect();
    let (filename, metric) = match args.len() {
        2 => (&args[1], Metric::Euclidean),
        3 => {
            let metric = match args[2].as_str() {
                "manhattan" => Metric::Manhattan,
                "euclidean" => Metric::Euclidean,
                _ => return Err(From::from("Unknown metric")),
            };

            (&args[1], metric)
        }
        _ => return Err(From::from("./compute_dist filename metric")),
    };

    // Read point_cloud data from the CSV.
    let mut point_cloud = Vec::new();

    let file = File::open(filename)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    for result in rdr.records() {
        let mut points = Vec::<f64>::new();
        let record = result?;
        for i in record.into_iter() {
            points.push(i.parse()?);
        }

        point_cloud.push(points);
    }

    // Compute the distance.
    let r = distance::dist(point_cloud, metric);
    for x in r {
        for y in x {
            print!("{},", y);
        }
        println!("");
    }

    Ok(())
}

fn main() {
    match run() {
        Err(e) => println!("{}", e),
        Ok(_) => (),
    }
}
