extern crate grouille;
extern crate itertools;
extern crate rand;
use grouille::{tycat, Point};
use itertools::repeat_call;
use rand::random;
use std::collections::HashSet;

type PointIndex = usize;


fn compute_dominating_points<I: Iterator<Item = Point>>(
    points: I,
    dominating_points: &mut HashSet<PointIndex>,
) {
    let mut points:Vec<Point> = points.collect();
    points.sort_unstable();
    let mut best_value = std::f64::NEG_INFINITY;
    let mut not_ok = HashSet::new();
    for (index, point) in points.iter().enumerate().rev() {
        let value = point.y - point.x;
        if value > best_value {
            best_value = value;
        } else {
            not_ok.insert(index);
        }
    }
    let mut best_value = std::f64::NEG_INFINITY;
    // TODO: check i can rev in O(1)
    for (index, point) in points.iter().enumerate() {
        let value = point.y + point.x;
        if value > best_value {
            best_value = value;
        } else {
            if not_ok.contains(&index) {
                dominating_points.insert(index);
            }
        }
    }
}

fn main() {
    let points: Vec<Point> = repeat_call(|| Point::new(random(), random()))
        .take(10)
        .collect();
    let mut dominating_points = HashSet::new();
    // TODO: sort only twice
    compute_dominating_points(points.iter().cloned(), &mut dominating_points);
    compute_dominating_points(
        points.iter().map(|p| Point::new(p.x, -p.y)),
        &mut dominating_points,
    );
    compute_dominating_points(
        points.iter().map(|p| Point::new(p.y, p.x)),
        &mut dominating_points,
    );
    compute_dominating_points(
        points.iter().map(|p| Point::new(p.y, -p.x)),
        &mut dominating_points,
    );
    let dpoints: Vec<_> = dominating_points.iter().map(|i| points[*i]).collect();
    tycat!(points, dpoints);
    println!(
        "we now have {} points instead of {} !",
        dpoints.len(),
        points.len()
    );
}
