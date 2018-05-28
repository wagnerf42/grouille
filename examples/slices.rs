#[macro_use]
extern crate grouille;

use grouille::Stl;

fn main() {
    let mut stl =
        Stl::new("test_files/cordoba.stl").expect("failed finding cordoba example stl file");
    let slices = stl.cut(0.3);
    for slice in &slices {
        tycat!(slice);
    }
}
