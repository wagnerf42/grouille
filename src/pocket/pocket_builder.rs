//! build polygons by looping on outer edges.
use crate::tycat::colored_display;
use elementary_path::ElementaryPath;
use pocket::Pocket;
use point::Point;
use std::collections::HashMap;

/// Converts elementary paths into oriented pockets (clockwise) by following edges.
/// Flat pockets are discarded in the process.
pub fn build_pockets(paths: Vec<ElementaryPath>) -> Vec<Pocket> {
    let mut points: HashMap<Point, Vec<ElementaryPath>> = HashMap::new();
    for path in paths {
        points
            .entry(path.start().clone())
            .or_insert_with(Vec::new)
            .push(path);
    }
    for neighbours in points.values_mut() {
        neighbours.sort_by_key(|p| p.start_angles());
    }

    (0..)
        .scan(points, |mut points, _| {
            if points.is_empty() {
                None
            } else {
                build_pocket(&mut points)
            }
        })
        .collect()
}

/// Builds pocket obtained when following path.
/// Discard flat ones and badly oriented ones.
fn build_pocket(points: &mut HashMap<Point, Vec<ElementaryPath>>) -> Option<Pocket> {
    let (starting_path, remove_point) = {
        let paths = points.values_mut().next().unwrap();
        (paths.pop().unwrap(), paths.is_empty())
    };
    if remove_point {
        points.remove(starting_path.start());
    }
    let mut edge = vec![starting_path];
    while edge.last().unwrap().end() != edge.first().unwrap().start() {
        edge.push(find_next_path(points, edge.last().unwrap()));
        tycat!(edge);
    }
    let pocket = Pocket::new(edge);
    if pocket.polygon_area() < 0.00001 {
        None
    } else {
        Some(pocket)
    }
}

fn find_next_path(
    points: &mut HashMap<Point, Vec<ElementaryPath>>,
    incoming_path: &ElementaryPath,
) -> ElementaryPath {
    let incoming_angles = incoming_path.end_angles();
    let index = points[incoming_path.end()]
        .binary_search_by_key(&incoming_angles, |p| p.start_angles())
        .unwrap_err()
        % points[incoming_path.end()].len();
    let next_path = points.get_mut(incoming_path.end()).unwrap().remove(index);
    if points[incoming_path.end()].is_empty() {
        points.remove(incoming_path.end());
    }
    next_path
}
