//! Provide a `Pocket` type.
use tycat::Tycat;
use {ElementaryPath, Point, Polygon, Quadrant};
pub mod pocket_builder;

/// `Polygon` equivalent, but also allowing arcs.
pub struct Pocket {
    /// All paths forming the pocket, one after the other.
    pub(crate) edge: Vec<ElementaryPath>,
    /// Quadrant containing us.
    pub(crate) quadrant: Quadrant,
}

impl Pocket {
    /// Build a new `Pocket` from given paths forming its edge.
    pub fn new(edge: Vec<ElementaryPath>) -> Self {
        let mut quadrant = Quadrant::new();
        for path in &edge {
            quadrant.update(&path.quadrant());
        }
        Pocket { edge, quadrant }
    }

    /// Iterate on all our points.
    pub fn points(&self) -> impl Iterator<Item = &Point> {
        self.edge.iter().map(|p| p.start())
    }

    /// Return the area of the polygon obtained when converting arcs to segments.
    pub fn polygon_area(&self) -> f64 {
        let polygon = Polygon::new(
            self.edge
                .iter()
                .map(|p| p.start().clone())
                .collect::<Vec<_>>(),
        );
        polygon.area()
    }
}
