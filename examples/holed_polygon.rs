#[macro_use]
extern crate grouille;
use grouille::{HoledPolygon, Point, Polygon};

fn main() {
    let outer_polygon = Polygon::new(vec![
        Point::new(10.0, 10.0),
        Point::new(300.0, 10.0),
        Point::new(150.0, 300.0),
    ]);
    let hole = Polygon::new(vec![
        Point::new(50.0, 50.0),
        Point::new(100.0, 120.0),
        Point::new(150.0, 80.0),
    ]);
    tycat!(outer_polygon, hole);
    let holed_polygon = HoledPolygon::new(outer_polygon, vec![hole]);
    tycat!(holed_polygon);
}
