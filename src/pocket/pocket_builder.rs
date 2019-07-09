//! build polygons by looping on outer edges.
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
        //TODO: this sort is incorrect
        neighbours.sort_by(|p1, p2| {
            p1.sweeping_angle()
                .partial_cmp(&p2.sweeping_angle())
                .unwrap()
        })
    }

    (0..)
        .scan(points, |mut points, _| {
            if points.is_empty() {
                None
            } else {
                Some(build_pocket(&mut points))
            }
        })
        .collect()
}

/// Builds pocket obtained when following path.
fn build_pocket(points: &mut HashMap<Point, Vec<ElementaryPath>>) -> Pocket {
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
    }
    Pocket::new(edge)
}

fn find_next_path(
    points: &mut HashMap<Point, Vec<ElementaryPath>>,
    incoming_path: &ElementaryPath,
) -> ElementaryPath {
    let incoming_angle = incoming_path.sweeping_angle();
    // TODO: the comparison is also not correct here
    let index = points[incoming_path.end()]
        .binary_search_by(|p| p.sweeping_angle().partial_cmp(&incoming_angle).unwrap())
        .unwrap();
    let next_path = points.get_mut(incoming_path.end()).unwrap().remove(index);
    if points[incoming_path.end()].is_empty() {
        points.remove(incoming_path.end());
    }
    next_path
}
