//! Provides `HoledPocket` structure.
use crate::Pocket;

/// pocket with (optional) holes inside.
#[derive(Debug)]
pub struct HoledPocket {
    /// Outer pocket (required to be oriented clockwise)
    pub outer_pocket: Pocket,
    /// Inner holes (required to be oriented counter clockwise)
    pub holes: Vec<Pocket>,
}

impl HoledPocket {
    /// Create a new holed polygon from given outer polygon and holes.
    pub fn new(outer_pocket: Pocket, holes: Vec<Pocket>) -> HoledPocket {
        debug_assert!(outer_pocket.polygon_area() > 0.0);
        debug_assert!(!holes.iter().any(|h| h.polygon_area() > 0.0));
        HoledPocket {
            outer_pocket,
            holes,
        }
    }
}
