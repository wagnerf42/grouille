//! functions to handle overlapping segments.
use std::collections::HashMap;
use {CoordinatesHash, ElementaryPath, HashKey, Point, Segment};

/// Remove overlapping segments.
pub fn remove_overlaps<S: IntoIterator<Item = Segment>>(segments: S) -> Vec<Segment> {
    let mut angles_hasher = CoordinatesHash::new(0.0001);
    let mut coordinates_hasher = CoordinatesHash::new(0.0001);
    let mut lines: HashMap<(HashKey, HashKey), HashMap<Point, isize>> = HashMap::new();
    // we start by hashing starting and ending points on each supporting line
    for segment in segments {
        let line_key = segment.line_key(&mut angles_hasher, &mut coordinates_hasher);
        let line_entry = lines.entry(line_key).or_insert_with(HashMap::new);
        for (i, p) in [1, -1].into_iter().zip(segment.ordered_points().iter()) {
            *line_entry.entry(*p).or_insert_with(|| 0) += i;
        }
    }
    // now loop on each line and keep all areas with a positive number of segments
    let mut non_overlapping_segments = Vec::new();
    for (_, mut aligned_points) in lines.drain() {
        let mut points: Vec<(Point, isize)> =
            aligned_points.drain().filter(|&(_, c)| c != 0).collect();
        points.sort_by_key(|&(p, _)| p);
        let mut remaining = points.into_iter();
        let (mut previous_point, mut count) = remaining.next().unwrap();
        for (point, count_change) in remaining {
            if count == 0 {
                previous_point = point;
            }
            count += count_change;
            if count == 0 {
                non_overlapping_segments.push(Segment::new(previous_point, point));
            }
        }
    }
    non_overlapping_segments
}

/// Remove all overlapping parts from our segments.
pub fn remove_segments_overlaps(
    paths: Vec<ElementaryPath>,
) -> impl Iterator<Item = ElementaryPath> {
    let mut elementary_segments = Vec::new();
    let mut remaining_paths = Vec::new();
    for path in paths {
        match path {
            ElementaryPath::Segment(s) => elementary_segments.push(s),
            a @ ElementaryPath::Arc(_) => remaining_paths.push(a),
        }
    }
    unimplemented!("TODO: we cannot call remove_overlaps here");
    // because these segments are ORIENTED
    remaining_paths.into_iter().chain(
        remove_overlaps(elementary_segments)
            .into_iter()
            .map(|s| ElementaryPath::Segment(s)),
    )
}
