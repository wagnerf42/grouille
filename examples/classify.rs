#[macro_use]
extern crate grouille;

use grouille::{
    classifier, overlap::remove_overlaps, polygon::polygon_builder::build_polygons, Stl,
};

fn main() {
    let mut stl = Stl::new("test_files/cordoba-very-large.stl")
        .expect("failed finding cordoba example stl file");
    let slice = stl.cut_at(1.2);
    tycat!(slice);
    let remaining_segments = remove_overlaps(&slice);
    tycat!(remaining_segments);
    let polygons = build_polygons(&remaining_segments);
    tycat!(&polygons);
    let (_, roots, _) = classifier::brute_force_classification(&polygons);
    println!("root polygons are: ");
    let root_polygons = roots.iter().map(|i| &polygons[*i]).collect::<Vec<_>>();
    tycat!(root_polygons);
}
