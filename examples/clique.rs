extern crate grouille;
extern crate itertools;
extern crate rand;
use grouille::{tycat, Point};
use itertools::repeat_call;
use rand::random;
use std::collections::HashSet;

type PointIndex = usize;

fn update_edge<I: Iterator<Item=PointIndex>, F: Fn(PointIndex)->f64>(edge_points: &mut HashSet<PointIndex>, points_indices: I, score_function : F) {
    let mut best_score = std::f64::NEG_INFINITY;
    for index in points_indices {
        let score = score_function(index);
        if score > best_score {
            best_score = score;
            edge_points.insert(index);
        }
    }
}

fn main() {
    let points: Vec<Point> = repeat_call(|| Point::new(random(), random()))
        .take(100)
        .collect();
    let mut horizontal_points:Vec<PointIndex> = (0..points.len()).collect();
    horizontal_points.sort_unstable_by(|&i1, &i2| points[i1].cmp(&points[i2]));

    let mut edge_points = HashSet::new();
    update_edge(&mut edge_points, horizontal_points.iter().cloned(), |i| points[i].x + points[i].y);
    update_edge(&mut edge_points, horizontal_points.iter().rev().cloned(), |i| -points[i].x + points[i].y);

    let dpoints: Vec<_> = edge_points.iter().map(|i| points[*i]).collect();
    tycat!(points, dpoints);
    println!(
        "we now have {} points instead of {} !",
        dpoints.len(),
        points.len()
    );
}
