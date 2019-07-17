//! common functions between `Polygon` and `Pocket` are abstracted through the `Shape` trait
//! defined here.
use crate::{ElementaryPath, Pocket, Point, Polygon, Quadrant, Segment};

/// Common abstration between `Polygon` and `Pocket`.
pub trait Shape {
    /// Return an y inside us with absolutely no points with this y value.
    fn inner_y(&self) -> f64;
    /// Return enclosing `Quadrant`.
    fn quadrant(&self) -> &Quadrant;
    /// Add all (intersections, index) at given y into given vector with given index.
    fn register_intersections(&self, intersections: &mut Vec<(f64, usize)>, index: usize, y: f64);
}

impl Shape for Polygon {
    fn inner_y(&self) -> f64 {
        let (y1, y2) = two_mins(self.points().iter().map(|p| p.y)).expect("flat shape");
        (y1 + y2) / 2.0
    }
    fn quadrant(&self) -> &Quadrant {
        &self.quadrant
    }
    fn register_intersections(&self, intersections: &mut Vec<(f64, usize)>, index: usize, y: f64) {
        intersections.extend(self.intersections_at_y(y).map(|i| (i, index)));
    }
}

impl Shape for Pocket {
    fn inner_y(&self) -> f64 {
        let (y1, y2) = two_mins(self.points().map(|p| p.y)).expect("flat shape");
        (y1 + y2) / 2.0
    }
    fn quadrant(&self) -> &Quadrant {
        &self.quadrant
    }
    fn register_intersections(&self, intersections: &mut Vec<(f64, usize)>, index: usize, y: f64) {
        let (xmin, xmax) = self.quadrant.limits(0);
        let s = ElementaryPath::Segment(Segment::new(
            Point::new(xmin - 0.1, y),
            Point::new(xmax + 0.1, y),
        ));
        intersections.extend(
            self.edge
                .iter()
                .flat_map(|p| p.intersections_with(&s).map(|i| (i.x, index))),
        );
    }
}

// Return the two minimal values (different) in the iterator.
fn two_mins<T: PartialOrd, I: Iterator<Item = T>>(iterator: I) -> Option<(T, T)> {
    let mut first = None;
    let mut second = None;
    for value in iterator {
        if let Some(ref first_value) = first {
            if let Some(ref second_value) = second {
                if value.partial_cmp(second_value).unwrap() == std::cmp::Ordering::Less
                    && value.partial_cmp(first_value).unwrap() != std::cmp::Ordering::Equal
                {
                    if value.partial_cmp(first_value).unwrap() == std::cmp::Ordering::Less {
                        second = first;
                        first = Some(value);
                    } else {
                        second = Some(value);
                    }
                }
            } else {
                match value.partial_cmp(first_value).unwrap() {
                    std::cmp::Ordering::Greater => second = Some(value),
                    std::cmp::Ordering::Less => {
                        second = first;
                        first = Some(value)
                    }
                    _ => (),
                }
            }
        } else {
            first = Some(value);
        }
    }
    if let Some(second_value) = second {
        first.map(|first_value| (first_value, second_value))
    } else {
        None
    }
}
