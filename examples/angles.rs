#[macro_use]
extern crate grouille;
use grouille::{Point, Segment};

fn main() {
    let o = Point::origin();
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
    let segments: Vec<Segment> = points.iter().map(|&p| Segment::new(o, p)).collect();

    for segment in &segments {
        let angle = segment.sweeping_angle();
        println!("angle de {:?} : {}", segment, angle.to_degrees());
        tycat!(o, segments, segment);
    }
}
