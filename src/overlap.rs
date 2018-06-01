//! functions to handle overlapping segments.
use std::collections::HashMap;
use {CoordinatesHash, HashKey, Point, Segment};

/// When an even number of segments overlap (even partially) keep none of them.
/// Will cut segments into smaller segments in case of partial overlap.
pub fn cut_even_overlaps(segments: &[Segment]) -> Vec<Segment> {
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
            if count % 2 == 0 {
                previous_point = point;
            }
            count += count_change;
            if count % 2 == 0 {
                non_overlapping_segments.push(Segment::new(previous_point, point));
            }
        }
    }
    non_overlapping_segments
}
