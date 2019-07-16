//! Module with different algorithms for computing intersections in a set of paths.
// use itertools::Itertools;
use itertools::Itertools;
use std::iter::{empty, once, repeat};
use std::mem;
use {ElementaryPath, Point, PointsHash};

/// Compute all intersections between given paths.
/// We return a vector such that for each path number i the cell number
/// i of the returned vector contain itself a vector of (possibly redundant) intersections on it.
fn compute_intersections(
    paths: &[ElementaryPath],
    points_hasher: &mut PointsHash,
) -> Vec<Option<Vec<Point>>> {
    let mut intersections: Vec<_> = repeat(None).take(paths.len()).collect();
    for (i1, p1) in paths.iter().enumerate() {
        for (i2, p2) in paths.iter().take(i1).enumerate() {
            for intersection in p1.intersections_with(p2) {
                let intersection = points_hasher.add(intersection);
                if intersections[i1].is_none() {
                    mem::replace(&mut intersections[i1], Some(Vec::new()));
                }
                intersections[i1].as_mut().unwrap().push(intersection);
                if intersections[i2].is_none() {
                    mem::replace(&mut intersections[i2], Some(Vec::new()));
                }
                intersections[i2].as_mut().unwrap().push(intersection);
            }
        }
    }
    intersections
}

/// Intersect given paths between themselves and return non intersecting subpaths (except on
/// endpoints).
pub fn intersect_paths(
    paths: &[ElementaryPath],
    points_hasher: &mut PointsHash,
) -> Vec<ElementaryPath> {
    let intersections = compute_intersections(paths, points_hasher);
    let result: Vec<ElementaryPath> = paths
        .iter()
        .zip(intersections.into_iter())
        .flat_map(|(path, path_intersections)| {
            let i = std::iter::Iterator::flatten(
                path_intersections
                    .map(|mut intersections| {
                        intersections.sort_unstable_by(|i1, i2| {
                            path.start()
                                .distance_to(i1)
                                .partial_cmp(&path.start().distance_to(i2))
                                .unwrap()
                        });
                        intersections
                    })
                    .into_iter(),
            );

            once(*path.start())
                .chain(i)
                .chain(once(*path.end()))
                .dedup()
                .tuple_windows()
                .map(move |(p1, p2)| path.sub_path(p1, p2))
        })
        .collect();
    result
}
