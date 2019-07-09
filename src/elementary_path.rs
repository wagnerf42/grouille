//! provides `ElementaryPath` structure for storing segments or arcs.
use crate::utils::{normalize_angle, Angle};
use std::f64::consts::{FRAC_PI_2, PI};
use {Arc, Point, PointsHash, Segment, Vector};

/// Elementary path (used for building larger paths)
/// can be either:
/// - `Arc`
/// - `Segment`
#[derive(Debug, Clone, Copy)]
pub enum ElementaryPath {
    /// `Arc` path
    Arc(Arc),
    /// `Segment` path
    Segment(Segment),
}

impl ElementaryPath {
    /// Create an `ElementaryPath` segment parallel to given one.
    /// at given distance and we can be on right or left side.
    pub fn parallel_segment(
        segment: &Segment,
        distance: f64,
        right_side: bool,
        rounder: &mut PointsHash,
    ) -> ElementaryPath {
        let direction = if right_side { 1.0 } else { -1.0 };
        let angle = (segment.end - segment.start).angle() + FRAC_PI_2 * direction;
        let displacement = Vector::polar(distance, angle);
        let start = segment.start + displacement;
        let end = segment.end + displacement;
        ElementaryPath::Segment(Segment::new(rounder.add(start), rounder.add(end)))
    }

    /// Return angles when leaving start point.
    /// We return a couple of angles.
    /// The first one is the tangent angle, the second
    /// one is the angle towards destination.
    pub fn start_angles(&self) -> (Angle, Angle) {
        let destination_angle = normalize_angle((self.end() - self.start()).angle());
        match *self {
            ElementaryPath::Segment(_) => (destination_angle, destination_angle),
            ElementaryPath::Arc(a) => {
                let tangent_angle = normalize_angle(PI - (a.start - a.center).angle());
                (tangent_angle, destination_angle)
            }
        }
    }

    /// Return angles when arriving at end point.
    /// We return a couple of angles.
    /// The first one is the tangent angle, the second
    /// one is the angle from start.
    pub fn end_angles(&self) -> (Angle, Angle) {
        let start_angle = normalize_angle((self.start() - self.end()).angle());
        match *self {
            ElementaryPath::Segment(_) => (start_angle, start_angle),
            ElementaryPath::Arc(a) => {
                let tangent_angle = normalize_angle(PI - (a.end - a.center).angle());
                (tangent_angle, start_angle)
            }
        }
    }

    /// Return length of underlying path.
    pub fn length(&self) -> f64 {
        match *self {
            ElementaryPath::Segment(ref s) => s.length(),
            ElementaryPath::Arc(ref a) => a.length(),
        }
    }

    /// Return ref on starting point.
    pub fn start(&self) -> &Point {
        match *self {
            ElementaryPath::Segment(ref s) => &s.start,
            ElementaryPath::Arc(ref a) => &a.start,
        }
    }

    /// Return ref on ending point.
    pub fn end(&self) -> &Point {
        match *self {
            ElementaryPath::Segment(ref s) => &s.end,
            ElementaryPath::Arc(ref a) => &a.end,
        }
    }

    /// Create a sub-path between given points.
    /// pre-condition: given points are on ourselves.
    pub fn sub_path(&self, start: Point, end: Point) -> ElementaryPath {
        match *self {
            ElementaryPath::Segment(_) => ElementaryPath::Segment(Segment::new(start, end)),
            ElementaryPath::Arc(ref a) => {
                ElementaryPath::Arc(Arc::new(start, end, a.center, a.radius))
            }
        }
    }

    /// Return endpoint which is not the given one.
    /// pre-condition: point given is an endpoint.
    pub fn other_endpoint(&self, endpoint: &Point) -> &Point {
        if self.start() == endpoint {
            self.end()
        } else {
            self.start()
        }
    }

    /// Iterate on all intersections (including possibly endpoints themselves)
    /// with other path.
    pub fn intersections_with<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item = Point> + 'a> {
        match *self {
            ElementaryPath::Arc(ref a) => match *other {
                ElementaryPath::Arc(ref a2) => {
                    Box::new(a.intersections_with_arc(a2)) as Box<Iterator<Item = Point>>
                }
                ElementaryPath::Segment(ref s2) => {
                    Box::new(a.intersections_with_segment(s2)) as Box<Iterator<Item = Point>>
                }
            },
            ElementaryPath::Segment(ref s) => match *other {
                ElementaryPath::Arc(ref a2) => {
                    Box::new(a2.intersections_with_segment(s)) as Box<Iterator<Item = Point>>
                }
                ElementaryPath::Segment(ref s2) => {
                    Box::new(s.intersection_with(s2).into_iter()) as Box<Iterator<Item = Point>>
                }
            },
        }
    }
}
