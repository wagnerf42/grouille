extern crate grouille;
extern crate itertools;
extern crate rand;

use grouille::{tycat::colored_display, Point};
use itertools::repeat_call;
use rand::random;
use std::collections::HashMap;

fn main() {
    let points: Vec<_> = repeat_call(|| Point::new(random(), random()))
        .take(1000)
        .collect();
    let mut squares = HashMap::new();
    for (index, point) in points.iter().enumerate() {
        let key = (
            (point.x / 0.5).floor() as usize,
            (point.y / 0.5).floor() as usize,
        );
        squares.entry(key).or_insert_with(Vec::new).push(index);
    }
    colored_display(
        squares
            .values()
            .map(|v| v.iter().map(|i| points[*i]).collect::<Vec<Point>>()),
    ).expect("display failed");
}
