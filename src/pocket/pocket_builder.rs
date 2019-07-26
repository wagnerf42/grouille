//! build polygons by looping on outer edges.
use crate::utils::iterators::GrouilleSlice;
use crate::utils::Angle;
use elementary_path::ElementaryPath;
use pocket::Pocket;
use point::Point;
use std::collections::HashMap;
use streaming_iterator::StreamingIterator;

/// Converts elementary paths into oriented pockets (clockwise) by following edges.
/// Flat pockets are discarded in the process.
pub fn build_pockets(paths: Vec<ElementaryPath>) -> Vec<Pocket> {
    let mut points: HashMap<Point, Vec<((Angle, Angle), Option<ElementaryPath>)>> = HashMap::new();
    // for path in crate::overlap::remove_segments_overlaps(paths) { // maybe we could avoid it
    // with a smarter key
    for path in paths {
        points
            .entry(path.end().clone())
            .or_insert_with(Vec::new)
            .push((path.end_angles(), None));
        points
            .entry(path.start().clone())
            .or_insert_with(Vec::new)
            .push((path.start_angles(), Some(path)));
    }
    for neighbours in points.values_mut() {
        neighbours.sort_by_key(|&(a, _)| a);
        debug_assert!(neighbours.wrapping_windows(2).all(|w| w[0].0 != w[1].0));
    }

    (0..)
        .scan(points, |mut points, _| {
            if points.is_empty() {
                None
            } else {
                Some(build_pocket(&mut points))
            }
        })
        .flatten()
        .collect()
}

/// Builds pocket obtained when following path.
/// Discard flat ones and badly oriented ones.
fn build_pocket(
    points: &mut HashMap<Point, Vec<((Angle, Angle), Option<ElementaryPath>)>>,
) -> Option<Pocket> {
    let starting_point = points.keys().next().expect("no starting point").clone();
    let incoming_angles: (Angle, Angle) = points[&starting_point]
        .iter()
        .filter_map(|(a, p)| if p.is_some() { None } else { Some(*a) })
        .next()
        .expect("no incoming edge on starting point");
    let starting_path = find_next_path(points, &starting_point, &incoming_angles);

    let mut edge = vec![starting_path];
    while edge.last().unwrap().end() != edge.first().unwrap().start() {
        edge.push(find_next_path(
            points,
            edge.last().unwrap().end(),
            &edge.last().unwrap().end_angles(),
        ));
    }
    let pocket = Pocket::new(edge);
    if pocket.polygon_area() < 0.00001 {
        None
    } else {
        Some(pocket)
    }
}

fn find_next_path(
    points: &mut HashMap<Point, Vec<((Angle, Angle), Option<ElementaryPath>)>>,
    current_point: &Point,
    incoming_angles: &(Angle, Angle),
) -> ElementaryPath {
    let paths = points.get_mut(current_point).unwrap();
    let incoming_index = paths
        .binary_search_by_key(incoming_angles, |&(a, _)| a)
        .unwrap();
    debug_assert!(paths[incoming_index].1.is_none());
    paths.remove(incoming_index);
    // loop on paths
    // we count the difference between arriving and leaving paths
    // when we reach +1 we leave through there
    let leaving_index = (0..paths.len())
        .map(|i| (i + incoming_index) % paths.len())
        .scan(0isize, |count, i| {
            if paths[i].1.is_some() {
                *count += 1;
            } else {
                *count -= 1;
            }
            Some((*count, i))
        })
        .find(|&(c, _)| c == 1)
        .map(|(_, i)| i)
        .expect("no leaving path");
    let leaving_path = paths.remove(leaving_index).1.unwrap();
    if points[current_point].is_empty() {
        points.remove(current_point);
    }
    leaving_path
}
