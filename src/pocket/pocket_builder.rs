//! build polygons by looping on outer edges.
use elementary_path::ElementaryPath;
use pocket::Pocket;
use point::Point;
use std::collections::HashMap;

/// Converts elementary paths into oriented pockets (clockwise) by following edges.
/// Flat pockets are discarded in the process.
pub fn build_pockets(paths: &[ElementaryPath]) -> Vec<Pocket> {
    let mut points: HashMap<&Point, Vec<&Point>> = HashMap::new();
    let mut remaining_paths = HashMap::new();
    for (path_id, path) in paths.iter().enumerate() {
        points
            .entry(path.start())
            .or_insert_with(Vec::new)
            .push(path.end());
        remaining_paths.insert(path_id, path);
    }
    for (point, neighbours) in &mut points {
        neighbours.sort_by(|&p1, &p2| {
            (p1 - *point)
                .angle()
                .partial_cmp(&(p2 - *point).angle())
                .unwrap()
        })
    }

    let mut pockets = Vec::new();
    while !remaining_paths.is_empty() {
        let next_start_id = remaining_paths.keys().next().cloned().unwrap();
        let next_start_path = remaining_paths.remove(&next_start_id).unwrap();
        if let Some(pocket) = build_pocket(next_start_path, &points, &mut remaining_paths) {
            pockets.push(pocket);
        }
    }
    pockets
}

/// Builds pocket obtained when following path. Might return None if obtained pocket is flat.
fn build_pocket(
    start_path: &ElementaryPath,
    points: &HashMap<&Point, Vec<&Point>>,
    remaining_paths: &mut HashMap<usize, &ElementaryPath>,
) -> Option<Pocket> {
    let starting_point = start_path.start();
    let mut previous_point = starting_point;
    let mut current_point = start_path.end();
    let mut pocket_edges = vec![start_path];
    unimplemented!()
    //        remaining_segments.remove(start_segment);
    //        //follow edge until we come back to our starting point
    //        while current_point != starting_point {
    //            let next_point = find_next_point(&points[&current_point], &current_point, &previous_point);
    //            remaining_segments.remove(&Segment::new(current_point, next_point));
    //            polygon_points.push(current_point);
    //            previous_point = current_point;
    //            current_point = next_point;
    //        }
    //        let polygon = Polygon::new(polygon_points);
    //        let area = polygon.area();
    //        //TODO: check which orientation we really want and adjust increment in find next accordingly
    //        if area < 0.00001 {
    //            // discard both flat and badly oriented polygons
    //            None
    //        } else {
    //            //keep only clockwise polygons
    //            Some(polygon.simplify())
    //        }
}

fn find_next_point(neighbours: &[Point], current_point: &Point, previous_point: &Point) -> Point {
    let incoming_angle = (previous_point - current_point).angle();
    let index = neighbours
        .binary_search_by(|p| {
            (p - current_point)
                .angle()
                .partial_cmp(&incoming_angle)
                .unwrap()
        })
        .unwrap();
    neighbours[(index + 1) % neighbours.len()]
}
