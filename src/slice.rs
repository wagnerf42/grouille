//! This module contains the `slice` function which is the main
//! entry point for slicers.
use holed_polygon::build_holed_polygons;
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
        let holed_polygons = build_holed_polygons(polygons);
        tycat!(&holed_polygons);
    }
    Ok(())
}
