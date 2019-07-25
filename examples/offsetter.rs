#[macro_use]
extern crate grouille;

use grouille::PointsHash;
use grouille::Polygon;

fn main() {
    let s = Polygon::square(0.0, 0.0, 50.0);
    let mut holes = vec![
        Polygon::square(20.0, 2.0, 10.0),
        //Polygon::square(20.0, 38.0, 10.0),
        //Polygon::square(2.0, 15.0, 20.0),
        //Polygon::square(28.0, 15.0, 20.0),
    ];

    holes.push(s);
    let holed_polygon = grouille::holed_polygon::build_holed_polygons(holes)
        .pop()
        .unwrap();
    println!("input: holed polygon");
    tycat!(holed_polygon);

    let mut hasher = PointsHash::new(0.000001);
    //holed_polygon.offset(3.0, &mut hasher); // TODO: debug me
    let pockets = holed_polygon.offset(2.0, &mut hasher);
    println!("holed pockets:");
    tycat!(pockets);
}
