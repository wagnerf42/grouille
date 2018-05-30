#[macro_use]
extern crate grouille;
use grouille::{Point, PointsHash, Segment};

fn main() {
    let mut hasher = PointsHash::new(0.0001);
    let segments = vec![
        Segment::new(
            hasher.add(Point::new(0.0, 0.0)),
            hasher.add(Point::new(1.0, 2.0)),
        ),
        Segment::new(
            hasher.add(Point::new(1.0, 2.0)),
            hasher.add(Point::new(2.0, 4.0)),
        ),
    ];
    let points = segments.iter().fold(Vec::new(), |mut v, s| {
        v.extend(s.ordered_points().iter().cloned());
        v
    });
    println!("before removing overlaping parts:");
    tycat!(segments, points);
}
