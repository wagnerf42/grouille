//! Provides `HoledPolygon` structure.
use Polygon;

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
}
