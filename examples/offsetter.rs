#[macro_use]
extern crate grouille;

use grouille::PointsHash;
use grouille::Polygon;

fn main() {
    let s = Polygon::square(0.0, 0.0, 50.0);
    let mut a = Polygon::square(20.0, 2.0, 10.0);
    let mut b = Polygon::square(20.0, 38.0, 10.0);
    let mut c = Polygon::square(2.0, 15.0, 20.0);
    let mut d = Polygon::square(28.0, 15.0, 20.0);
    tycat!(s, a, b, c, d);

    let holed_polygon = grouille::holed_polygon::build_holed_polygons(vec![s, a, b, c, d])
        .pop()
        .unwrap();
    tycat!(holed_polygon);

    let mut hasher = PointsHash::new(0.000001);
    holed_polygon.offset(6.0, &mut hasher);
}
