#[macro_use]
extern crate grouille;

use grouille::PointsHash;
use grouille::Polygon;
use grouille::{HoledPolygon, Point, Quadrant};

fn main() {
    let outer_polygon = Polygon::new(vec![
        Point {
            x: 1.5,
            y: -0.2071000039577484,
        },
        Point { x: 1.0, y: 0.0 },
        Point {
            x: 1.5,
            y: 0.2071000039577484,
        },
        Point {
            x: 1.1715999841690063,
            y: 1.0,
        },
        Point {
            x: 1.2928999662399292,
            y: 1.2928999662399292,
        },
        Point {
            x: 1.0,
            y: 1.1715999841690063,
        },
        Point {
            x: 0.2071000039577484,
            y: 1.5,
        },
        Point { x: 0.0, y: 1.0 },
        Point {
            x: -0.2071000039577484,
            y: 1.5,
        },
        Point {
            x: -1.0,
            y: 1.1715999841690063,
        },
        Point {
            x: -1.2928999662399292,
            y: 1.2928999662399292,
        },
        Point {
            x: -1.1715999841690063,
            y: 1.0,
        },
        Point {
            x: -1.5,
            y: 0.2071000039577484,
        },
        Point { x: -1.0, y: 0.0 },
        Point {
            x: -1.5,
            y: -0.2071000039577484,
        },
        Point {
            x: -1.1715999841690063,
            y: -1.0,
        },
        Point {
            x: -1.2928999662399292,
            y: -1.2928999662399292,
        },
        Point {
            x: -1.0,
            y: -1.1715999841690063,
        },
        Point {
            x: -0.2071000039577484,
            y: -1.5,
        },
        Point { x: 0.0, y: -1.0 },
        Point {
            x: 0.2071000039577484,
            y: -1.5,
        },
        Point {
            x: 1.0,
            y: -1.1715999841690063,
        },
        Point {
            x: 1.2928999662399292,
            y: -1.2928999662399292,
        },
        Point {
            x: 1.1715999841690063,
            y: -1.0,
        },
    ]);
    let holes = vec![
        Polygon::new(vec![
            Point { x: 1.0, y: -1.05 },
            Point {
                x: 0.9121299922466278,
                y: -1.0878699898719788,
            },
            Point {
                x: 0.9493238099930883,
                y: -1.0,
            },
            Point {
                x: 0.9121299922466278,
                y: -0.9121299922466278,
            },
            Point {
                x: 1.0,
                y: -0.9493238099930883,
            },
            Point {
                x: 1.0864994217216073,
                y: -0.9121299922466278,
            },
            Point {
                x: 1.050676130402267,
                y: -1.0,
            },
            Point {
                x: 1.0864994217216073,
                y: -1.0878699898719788,
            },
        ]),
        Polygon::new(vec![
            Point {
                x: 0.746799362869739,
                y: -1.0,
            },
            Point {
                x: 0.6484967290208511,
                y: -1.2372364069084152,
            },
            Point {
                x: 0.24348583021761763,
                y: -1.4050108913689072,
            },
            Point {
                x: 0.09439198302118987,
                y: -1.0466011187912823,
            },
            Point {
                x: 0.0757113178377663,
                y: -1.0,
            },
            Point {
                x: 0.24348583021761763,
                y: -0.5949891146045607,
            },
            Point {
                x: 0.6484967290208511,
                y: -0.7627635930915848,
            },
        ]),
    ];
    let holed_polygon = HoledPolygon::new(outer_polygon, holes);

    //    let s = Polygon::square(0.0, 0.0, 50.0);
    //    let mut holes = vec![
    //        Polygon::square(20.0, 2.0, 10.0),
    //        //Polygon::square(20.0, 38.0, 10.0),
    //        //Polygon::square(2.0, 15.0, 20.0),
    //        //Polygon::square(28.0, 15.0, 20.0),
    //    ];
    //
    //    holes.push(s);
    //    let holed_polygon = grouille::holed_polygon::build_holed_polygons(holes)
    //        .pop()
    //        .unwrap();
    println!("input: holed polygon");
    tycat!(holed_polygon);

    let mut hasher = PointsHash::new(0.001);
    //holed_polygon.offset(3.0, &mut hasher); // TODO: debug me
    let pockets = holed_polygon.offset(0.1, &mut hasher);
    println!("holed pockets:");
    tycat!(pockets);
}
