extern crate grouille;
extern crate rand;
extern crate streaming_iterator;
use grouille::tycat;
use grouille::utils::iterators::GrouilleSlice;
use grouille::Point;
use grouille::Segment;
use rand::random;
use std::f64::consts::PI;
use std::iter::repeat_with;
use streaming_iterator::StreamingIterator;

fn random_point() -> Point {
    Point::new(random(), random())
}

fn random_segment() -> Segment {
    Segment::new(random_point(), random_point())
}

fn main() {
    // create random segments
    let initial_segments: Vec<_> = repeat_with(random_segment).take(5).collect();
    tycat!(initial_segments);

    // rotate them 8 times
    let center = Point::new(0.5, 0.5);
    let segments = &initial_segments;
    let rotated_segments: Vec<Segment> = (0..8)
        .map(|i| PI / 4.0 * (i as f64))
        .flat_map(move |a| segments.iter().map(move |s| s.rotate_around(&center, a)))
        .collect();
    tycat!(rotated_segments);

    // intersect with unit square
    let square_points = [
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.0),
        Point::new(1.0, 1.0),
        Point::new(0.0, 1.0),
    ];
    let mut square = Vec::new();
    square_points
        .wrapping_windows(2)
        .map(|points| Segment::new(points[0], points[1]))
        .for_each(|s| square.push(s.clone()));
    tycat!(square, rotated_segments);
}
