// distance/src/lib.rs

pub enum Metric {
    Manhattan,
    Euclidean,
}

/// Given a point cloud (a Vector of Vectors containing point data) and a
/// metric, compute a distance matrix for the entirety.
pub fn dist(point_cloud: Vec<Vec<f64>>, metric: Metric) -> Vec<Vec<f64>> {
    let mut r = Vec::new();

    let f = match metric {
        Metric::Manhattan => manhattan_distance,
        Metric::Euclidean => euclidean_distance,
    };

    // TODO: This can be minimized down from O(n^2) to O(nlogn), see a
    // triangular distance matrix.
    for x in &point_cloud {
        let mut tmp = Vec::new();
        for y in &point_cloud {
            tmp.push(f(x, y));
        }
        r.push(tmp)
    }

    r
}

/// Compute the manhattan distance between two f64 vectors.
fn manhattan_distance(p: &[f64], q: &[f64]) -> f64 {
    p.iter()
        .zip(q.iter())
        .fold(0.0, |acc, (x, y)| acc + (x - y).abs())
}

/// Compute the euclidean distance between two f64 vectors.
fn euclidean_distance(p: &[f64], q: &[f64]) -> f64 {
    p.iter()
        .zip(q.iter())
        .fold(0.0, |acc, (x, y)| acc + ((x - y) * (x - y)))
        .sqrt()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_dist() {
        let a = vec![vec![1.0], vec![2.0], vec![3.0]];
        let b = vec![
            vec![0.0, 1.0, 2.0],
            vec![1.0, 0.0, 1.0],
            vec![2.0, 1.0, 0.0],
        ];
        assert_eq!(dist(a, Metric::Euclidean), b)
    }

    #[test]
    fn test_manhattan() {
        let a = vec![1.0];
        let b = vec![1.0];
        assert_eq!(manhattan_distance(&a, &b), 0.0);

        let a = vec![3.0, 2.0, 1.0];
        let b = vec![1.0, 2.0, 3.0];
        assert_eq!(manhattan_distance(&a, &b), 4.0);

        let a = vec![3.4, 2.1, 6.6];
        let b = vec![-3.2, -55.0, 100.0];
        assert_eq!(manhattan_distance(&a, &b), 157.10000000000002);
    }

    #[test]
    fn test_euclidean() {
        let a = vec![1.0];
        let b = vec![1.0];
        assert_eq!(euclidean_distance(&a, &b), 0.0);

        let a = vec![3.0, 2.0, 1.0];
        let b = vec![1.0, 2.0, 3.0];
        assert_eq!(euclidean_distance(&a, &b), 2.8284271247461903);

        let a = vec![3.4, 2.1, 6.6];
        let b = vec![-3.2, -55.0, 100.0];
        assert_eq!(euclidean_distance(&a, &b), 109.67009619764178);
    }
}
