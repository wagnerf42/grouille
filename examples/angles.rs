#[macro_use]
extern crate grouille;
use grouille::{Point, PointsHash, Segment};

fn main() {
    let mut hasher = PointsHash::new(0.00001);
    let o = hasher.add(Point::origin());
    let points = [
        Point::new(1.0, 0.0),
        Point::new(1.0, 0.05),
        Point::new(1.0, 1.0),
        Point::new(0.0, 1.0),
        Point::new(-1.0, 1.0),
        Point::new(-1.0, 0.05),
        Point::new(-1.0, 0.0),
        Point::new(-1.0, -1.0),
    ];
    let segments: Vec<Segment> = points
        .iter()
        .map(|&p| Segment::new(o, hasher.add(p)))
        .collect();

    for segment in &segments {
        let angle = segment.sweeping_angle();
        println!("angle de {:?} : {}", segment, angle.to_degrees());
        tycat!(o, segments, segment);
    }
}
