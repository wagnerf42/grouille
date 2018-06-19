//! Provides `HoledPolygon` structure.
use classifier::brute_force_classification;
use intersections::intersect_paths;
use itertools::repeat_call;
use std::iter::repeat;
use {PointsHash, Polygon};

/// polygon with (optional) holes inside.
#[derive(Debug)]
pub struct HoledPolygon {
    /// Outer polygon (required to be oriented clockwise)
    pub outer_polygon: Polygon,
    /// Inner holes (required to be oriented counter clockwise)
    pub holes: Vec<Polygon>,
}

impl HoledPolygon {
    /// Create a new holed polygon from given outer polygon and holes.
    pub fn new(outer_polygon: Polygon, holes: Vec<Polygon>) -> HoledPolygon {
        assert!(outer_polygon.is_oriented_clockwise());
        assert!(!holes.iter().any(|h| h.is_oriented_clockwise()));
        HoledPolygon {
            outer_polygon,
            holes,
        }
    }

    /// Offset holed polygon into holed pockets
    pub fn offset(&self, radius: f64, points_hasher: &mut PointsHash) {
        let mut paths = self.outer_polygon.inner_paths(radius, points_hasher);
        for hole in &self.holes {
            paths.append(&mut hole.inner_paths(radius, points_hasher));
        }
        tycat!(self.outer_polygon, paths);
        let small_paths = intersect_paths(&paths, points_hasher);
        tycat!(self.outer_polygon, small_paths);
    }
}

/// Turn some `Polygon`s into `HoledPolygon`s by classifiying who is inside whom.
pub fn build_holed_polygons(polygons: Vec<Polygon>) -> Vec<HoledPolygon> {
    let polygons_number = polygons.len();
    let (_, roots, fathers) = brute_force_classification(&polygons);
    // now build the inclusion tree in the right direction (towards children not towards fathers)
    let mut children: Vec<Vec<usize>> = repeat_call(Vec::new).take(polygons_number).collect();

    for index in 0..polygons_number {
        if fathers[index] != index as isize {
            children[fathers[index] as usize].push(index);
        }
    }

    // compute level in tree for each node
    let mut depths: Vec<usize> = repeat(0).take(polygons_number).collect(); // what depth in inclusion tree is each polygon ?
    fn set_depth(root: usize, depth: usize, depths: &mut [usize], children: &[Vec<usize>]) {
        assert_eq!(depths[root], 0); // should not change twice
        depths[root] = depth;
        for child in &children[root] {
            set_depth(*child, depth + 1, depths, children);
        }
    }

    for root in roots {
        set_depth(root, 0, &mut depths, &children);
    }

    // polygons at even levels are outer part of holed polygon
    // at odd levels they are holes
    let mut holed_polygons_parts: Vec<(Option<Polygon>, Vec<Polygon>)> = repeat_call(|| {
        (None, Vec::new())
    }).take(polygons_number)
        .collect();
    for (index, mut polygon) in polygons.into_iter().enumerate() {
        if depths[index] % 2 == 0 {
            holed_polygons_parts[index].0 = Some(polygon);
        } else {
            holed_polygons_parts[fathers[index] as usize].1.push({
                polygon.reverse();
                polygon
            });
        }
    }
    holed_polygons_parts
        .into_iter()
        .filter_map(|(polygon, children)| polygon.map(|p| HoledPolygon::new(p, children)))
        .collect()
}
