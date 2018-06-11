//! This module contains the `slice` function which is the main
//! entry point for slicers.
use classifier;
use overlap::remove_overlaps;
use polygon::polygon_builder::build_polygons;
use std::io::Error;
use std::path::Path;
use Stl;

/// Load stl file and slice it.
pub fn slice<P: AsRef<Path>>(stl_file: P, thickness: f64) -> Result<(), Error> {
    let mut stl = Stl::new(stl_file)?;
    let slices = stl.cut(thickness);
    for slice in &slices {
        let remaining_segments = remove_overlaps(slice);
        let polygons = build_polygons(&remaining_segments);
        tycat!(&polygons);
        let (classified, roots) = classifier::brute_force_classification(&polygons);
        println!("root polygons are: ");
        let root_polygons = roots.iter().map(|i| &polygons[*i]).collect::<Vec<_>>();
        tycat!(root_polygons);
    }
    Ok(())
}
