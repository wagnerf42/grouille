#[macro_use]
extern crate grouille;
extern crate itertools;
extern crate rand;

use grouille::Point;
use itertools::repeat_call;
use rand::random;

fn main() {
    let points: Vec<_> = repeat_call(|| Point::new(random(), random()))
        .take(1000)
        .collect();
    let o = Point::new(0.0, 0.0);
    tycat!(o, points);
}
