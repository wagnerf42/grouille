//! This module allows for the initial classification
//! of polygons into a polygon tree.
use tycat::Tycat;
use {Polygon, Quadrant};

enum Status<'a> {
    Root,
    Child(&'a Polygon),
}

struct CPolygon<'a> {
    polygon: &'a Polygon,
    status: Status<'a>,
    level: usize, // level in tree
}

/// Take a polygons slice.
/// Return a vec<(father,level)> indicating
/// for each polygon his father's index and its level in the
/// tree. Root polygons are at level 0.
/// pre-condition: no overlap, even on points.
pub fn classify_polygons(polygons: &[Polygon]) -> Vec<(usize, usize)> {
    let remaining_polygons: Vec<_> = polygons
        .iter()
        .map(|p| CPolygon {
            polygon: p,
            status: Status::Root,
            level: 0,
        })
        .collect();
    let global_quadrant = polygons.iter().fold(Quadrant::new(), |mut q, p| {
        q.update(&p.quadrant);
        q
    });
    unimplemented!()
}

/// Take some polygons to classify and return two vectors of polygons : roots and others
fn brute_force_classification(polygons: Vec<CPolygon>) -> (Vec<CPolygon>, Vec<CPolygon>) {
    let mut remaining_polygons = polygons;
    unimplemented!()
}
