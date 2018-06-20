//! Provide a `Pocket` type.
use {ElementaryPath, Quadrant};
use tycat::Tycat;

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
}
