#[macro_use]
extern crate grouille;

use grouille::{PointsHash, Stl};

fn main() {
    let mut stl =
        Stl::new("test_files/cordoba.stl").expect("failed finding cordoba example stl file");
    let mut points_hasher = PointsHash::new(0.001);
    let slices = stl.cut(0.3, &mut points_hasher);
    for slice in &slices {
        tycat!(slice);
    }
}
