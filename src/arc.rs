//! provides the `Arc` class.
use std::f64::consts::{FRAC_PI_2, PI};
use std::iter::{empty, once};
use utils::is_almost;
use {Point, Segment};

/// Oriented arc segment. Always less than a half circle.
#[derive(Debug, Clone, Copy)]
pub struct Arc {
    /// Starting point (hashed)
    pub start: Point,
    /// Ending point (hashed)
    pub end: Point,
    /// Center (not hashed)
    pub center: Point,
    /// Radius
    pub radius: f64,
}

impl Arc {
    /// Create a new arc.
    pub fn new(start: Point, end: Point, center: Point, radius: f64) -> Arc {
        assert!(start != end);
        let mut arc = Arc {
            start,
            end,
            center,
            radius,
        };
        if !(is_almost(arc.center.distance_to(&arc.start), arc.radius)
            && is_almost(arc.center.distance_to(&arc.end), arc.radius))
        {
            arc.adjust_center();
        }
        arc
    }

    /// Given center was not completely right, move it slightly.
    /// This can happen for example when endpoints have been rounded.
    fn adjust_center(&mut self) {
        self.center = self
            .possible_centers()
            .into_iter()
            .min_by(|c1, c2| {
                c1.distance_to(&self.center)
                    .partial_cmp(&c2.distance_to(&self.center))
                    .expect("failed comparing center distances")
            })
            .expect("no center found");
    }

    /// Return array of the two centers we could have.
    fn possible_centers(&self) -> Vec<Point> {
        // we do some geometry to avoid too complex equations.
        // take start as origin
        let support = self.end - self.start;
        let middle = self.start.center_with(&self.end);
        // find bisector
        let bisector_point = middle + support.perpendicular_vector();
        let line = Segment::new(middle, bisector_point);
        let centers = line_circle_intersections(&line, &self.start, self.radius).collect();
        centers
    }

    /// Return normalized angle of points with center.
    pub fn angle(&self) -> f64 {
        ((self.start - self.center).angle() - (self.end - self.center).angle()) % (2.0 * PI)
    }

    /// Return the arc's length.
    pub fn length(&self) -> f64 {
        let angle = self.angle();
        if angle > PI {
            ((2.0 * PI) - angle) * self.radius
        } else {
            angle * self.radius
        }
    }

    /// Do we contain given point ?
    pub fn contains(&self, point: &Point) -> bool {
        if self.start.is_almost(point) || self.end.is_almost(point) {
            true
        } else if is_almost(self.center.distance_to(point), self.radius) {
            self.contains_circle_point(point)
        } else {
            false
        }
    }

    /// Do we contain given point on circle but not as endpoint ?
    pub fn strictly_contains(&self, point: &Point) -> bool {
        if self.start.is_almost(point) || self.end.is_almost(point) {
            false
        } else {
            self.contains_circle_point(point)
        }
    }

    /// Do we contain given point which is on our circle ?
    pub fn contains_circle_point(&self, point: &Point) -> bool {
        let s = Segment::new(self.start, self.end);
        let s2 = Segment::new(self.center, *point);
        s.intersection_with(&s2).is_some()
    }

    /// Intersect ourselves with horizontal line at given y.
    /// pre-condition: there is exactly one intersection
    pub fn horizontal_line_intersection(&self, y: f64) -> Point {
        // we use pythagoras
        let side_length = (y - self.center.y).abs();
        if is_almost(side_length, self.radius) {
            return Point::new(self.center.x, y);
        }
        if side_length > self.radius {
            panic!("no arc hline intersection");
        }
        let other_side_length = (self.radius * self.radius - side_length * side_length).sqrt();
        let candidate_point = Point::new(self.center.x - other_side_length, y);
        if self.contains_circle_point(&candidate_point) {
            candidate_point
        } else {
            let candidate_point2 = Point::new(self.center.x + other_side_length, y);
            if self.contains_circle_point(&candidate_point2) {
                candidate_point2
            } else {
                panic!("no arc hline");
            }
        }
    }

    /// Return angle for tangent at given point.
    /// pre-condition: we contain given point.
    pub fn tangent_angle(&self, tangent_point: &Point) -> f64 {
        let base_angle = (tangent_point - self.center).angle();
        (base_angle + FRAC_PI_2) % PI
    }

    /// Iterate on all points obtained when intersecting with given Arc.
    pub fn intersections_with_arc<'a>(
        &'a self,
        other: &'a Self,
    ) -> impl Iterator<Item = Point> + 'a {
        circles_intersections(&self.center, &other.center, self.radius, other.radius)
            .filter(move |p| self.contains_circle_point(p) && other.contains_circle_point(p))
    }

    /// Iterate on all points obtained when intersecting with given Segment.
    pub fn intersections_with_segment<'a>(
        &'a self,
        other: &'a Segment,
    ) -> impl Iterator<Item = Point> + 'a {
        line_circle_intersections(other, &self.center, self.radius)
            .filter(move |p| self.contains_circle_point(p) && other.contains(p))
    }
}

fn line_circle_intersections<'a>(
    segment: &'a Segment,
    center: &'a Point,
    radius: f64,
) -> impl Iterator<Item = Point> + 'a {
    let d = segment.end - segment.start;
    let c = center - segment.start;
    // segment points are at alpha * d
    // distance(alpha * d, center) = r

    // (xc-alpha*xd)**2 + (yc-alpha*yd)**2 - r**2 = 0

    // xc**2 + alpha**2*xd**2 -2*alpha*xc*xd
    // yc**2 + alpha**2*yd**2 -2*alpha*yc*yd
    // - r**2 = 0
    let a = d.x * d.x + d.y * d.y;
    let b = (c.x * d.x + c.y * d.y) * (-2.0);
    let c = c.x * c.x + c.y * c.y - radius * radius;
    let solutions = solve_quadratic_equation(a, b, c);
    solutions.into_iter().map(move |s| segment.start + d * s)
}

fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> Vec<f64> {
    let delta = b * b - a * c * 4.0;
    if is_almost(delta.abs().sqrt(), 0.0) {
        if is_almost(a, 0.0) {
            Vec::new()
        } else {
            vec![-b / (a * 2.0)]
        }
    } else if delta < 0.0 {
        Vec::new()
    } else {
        vec![
            (-b - delta.sqrt()) / (a * 2.0),
            (-b + delta.sqrt()) / (a * 2.0),
        ]
    }
}

fn circles_intersections(
    c1: &Point,
    c2: &Point,
    r1: f64,
    r2: f64,
) -> Box<dyn Iterator<Item = Point>> {
    // I just solved all equations to end up with this.
    let d = c1.distance_to(c2);
    if is_almost(d, 0.0) {
        Box::new(empty()) // common center
    } else {
        let l = if is_almost(r1, r2) {
            d / 2.0
        } else {
            (r1 * r1 - r2 * r2) / (d * 2.0) + d / 2.0
        };

        if is_almost(r1, l) {
            // only one intersection
            Box::new(once(Point::new(
                l / d * (c2.x - c1.x) + c1.x,
                l / d * (c2.y - c1.y) + c1.y,
            )))
        } else if (r1 < l) || (r1.abs() < l.abs()) {
            Box::new(empty()) // too far away
        } else {
            let h = (r1 * r1 - l * l).sqrt();
            let p1 = Point::new(
                l / d * (c2.x - c1.x) + h / d * (c2.y - c1.y) + c1.x,
                l / d * (c2.y - c1.y) - h / d * (c2.x - c1.x) + c1.y,
            );
            let p2 = Point::new(
                l / d * (c2.x - c1.x) - h / d * (c2.y - c1.y) + c1.x,
                l / d * (c2.y - c1.y) + h / d * (c2.x - c1.x) + c1.y,
            );
            if p1.is_almost(&p2) {
                Box::new(once(p1))
            } else {
                Box::new(once(p1).chain(once(p2)))
            }
        }
    }
}
