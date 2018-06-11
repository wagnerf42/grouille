#[macro_use]
extern crate grouille;
use grouille::{overlap::remove_overlaps, Point, PointsHash, Segment};

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
        Segment::new(
            hasher.add(Point::new(0.5, 1.0)),
            hasher.add(Point::new(1.5, 3.0)),
        ),
        Segment::new(
            hasher.add(Point::new(0.0, 0.0)),
            hasher.add(Point::new(1.0, 0.0)),
        ),
        Segment::new(
            hasher.add(Point::new(2.0, 0.0)),
            hasher.add(Point::new(3.0, 0.0)),
        ),
    ];
    let points = segments.iter().fold(Vec::new(), |mut v, s| {
        v.extend(s.ordered_points().iter().cloned());
        v
    });
    println!("before removing overlaping parts:");
    tycat!(segments, points);
    let remaining_segments = remove_overlaps(&segments);

    println!("after:");
    tycat!(remaining_segments, points);
}
