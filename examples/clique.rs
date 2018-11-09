extern crate grouille;
extern crate itertools;
extern crate rand;
use grouille::{tycat, Point};
use itertools::repeat_call;
use rand::random;
use std::cmp::Ordering;
use std::collections::HashSet;

type PointIndex = usize;

fn update_side_one_pass<'a, I: Iterator<Item = &'a PointIndex>, F: Fn(&PointIndex) -> f64>(
    edge_points: &mut HashSet<PointIndex>,
    points_indices: I,
    score_function: F,
) {
    let mut best_score = std::f64::NEG_INFINITY;
    for index in points_indices {
        let score = score_function(index);
        if score > best_score {
            best_score = score;
            edge_points.insert(*index);
        }
    }
}

fn update_side<X, Y>(
    edge_points: &mut HashSet<PointIndex>,
    indices: &mut [PointIndex],
    score_x: X,
    score_y: Y,
) where
    X: Fn(&PointIndex) -> f64,
    Y: Fn(&PointIndex) -> f64,
{
    indices.sort_unstable_by(|i1, i2| {
            score_x(i1)
                .partial_cmp(&score_x(i2))
                .unwrap()
                .then(score_y(i1).partial_cmp(&score_y(i2)).unwrap())
    });
    update_side_one_pass(edge_points, indices.iter(), |i| score_x(i) + score_y(i));
    update_side_one_pass(edge_points, indices.iter().rev(), |i| {
        -score_x(i) + score_y(i)
    });
}

fn main() {
    let points: Vec<Point> = repeat_call(|| Point::new(random(), random()))
        .take(10_000)
        .collect();

    let mut edge_points = HashSet::new();
    let mut indices: Vec<PointIndex> = (0..points.len()).collect();
    update_side(
        &mut edge_points,
        &mut indices,
        |i| points[*i].x,
        |i| points[*i].y,
    );
    update_side(
        &mut edge_points,
        &mut indices,
        |i| points[*i].x,
        |i| -points[*i].y,
    );
    update_side(
        &mut edge_points,
        &mut indices,
        |i| points[*i].y,
        |i| points[*i].x,
    );
    update_side(
        &mut edge_points,
        &mut indices,
        |i| points[*i].y,
        |i| -points[*i].x,
    );


    let dpoints: Vec<_> = edge_points.iter().map(|i| points[*i]).collect();
    tycat!(points, dpoints);
    println!(
        "we now have {} points instead of {} !",
        dpoints.len(),
        points.len()
    );
}
