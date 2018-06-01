//! Polygon module.
use Point;

/// Oriented polygons.
#[derive(Debug)]
pub struct Polygon {
    /// Vector of all points forming the edge of the polygon.
    pub points: Vec<Point>,
}

impl Polygon {
    /// Create polygon out of given points vector.
    pub fn new(points: Vec<Point>) -> Polygon {
        Polygon { points }
    }
    /// Returns area taken by polygon.
    /// Negative or Positive depending on orientation.
    pub fn area(&self) -> f64 {
        self.points
            .iter()
            .zip(self.points.iter().cycle().skip(1))
            .map(|(p1, p2)| p1.x * p2.y - p1.y * p2.x)
            .sum::<f64>() / 2.0
    }

    /// Returns if polygon is oriented clockwise (with respect to svg
    /// orientation)
    pub fn is_oriented_clockwise(&self) -> bool {
        self.area() > 0.0
    }

    /// Simplifies polygon by removing points
    /// without losing too much precision.
    ///
    /// # Example
    /// ```
    /// use grouille::{Point, Polygon};
    /// //note: you can add some tycat! to visualize the example.
    ///
    /// let complex_polygon = Polygon::new(
    ///     vec![
    ///     Point::new(-1.5, 0.2071000039577484),
    ///     Point::new(-1.29497096657753, 0.7020999744534493),
    ///     Point::new(-1.2928999662399292, 0.707099974155426),
    ///     Point::new(-1.1728129839897157, 0.9970709997415542),
    ///     Point::new(-1.1715999841690063, 1.0),
    ///     Point::new(-1.1728129839897157, 1.0029289996623993),
    ///     Point::new(-1.2928999662399292, 1.2928999662399292),
    ///     Point::new(-1.0029289996623993, 1.1728129839897157),
    ///     Point::new(-1.0, 1.1715999841690063),
    ///     Point::new(-0.7100289744138718, 1.2916869664192199),
    ///     Point::new(-0.707099974155426, 1.2928999662399292),
    ///     Point::new(-0.2121000036597252, 1.4979289996623992),
    ///     Point::new(-0.2071000039577484, 1.5),
    ///     Point::new(-0.002071000039577484, 1.005),
    ///     Point::new(0.0, 1.0),
    ///     Point::new(0.20502900391817092, 1.495),
    ///     Point::new(0.2071000039577484, 1.5),
    ///     Point::new(0.7020999744534493, 1.29497096657753),
    ///     Point::new(0.707099974155426, 1.2928999662399292),
    ///     Point::new(0.9970709997415542, 1.1728129839897157),
    ///     Point::new(1.0, 1.1715999841690063),
    ///     Point::new(1.2899709665775299, 1.2916869664192199),
    ///     Point::new(1.2928999662399292, 1.2928999662399292),
    ///     Point::new(1.2916869664192199, 1.2899709665775299),
    ///     Point::new(1.1715999841690063, 1.0),
    ///     Point::new(1.2916869664192199, 0.7100289744138718),
    ///     Point::new(1.2928999662399292, 0.707099974155426),
    ///     Point::new(1.4979289996623992, 0.2121000036597252),
    ///     Point::new(1.5, 0.2071000039577484),
    ///     Point::new(1.495, 0.20502900391817092),
    ///     Point::new(1.0, 0.0),
    ///     Point::new(1.495, -0.20502900391817092),
    ///     Point::new(1.5, -0.2071000039577484),
    ///     Point::new(1.4979289996623992, -0.2121000036597252),
    ///     Point::new(1.2928999662399292, -0.707099974155426),
    ///     Point::new(1.2916869664192199, -0.7100289744138718),
    ///     Point::new(1.1715999841690063, -1.0),
    ///     Point::new(1.2916869664192199, -1.2899709665775299),
    ///     Point::new(1.2928999662399292, -1.2928999662399292),
    ///     Point::new(1.2899709665775299, -1.2916869664192199),
    ///     Point::new(1.0, -1.1715999841690063),
    ///     Point::new(0.9970709997415542, -1.1728129839897157),
    ///     Point::new(0.707099974155426, -1.2928999662399292),
    ///     Point::new(0.7020999744534493, -1.29497096657753),
    ///     Point::new(0.2071000039577484, -1.5),
    ///     Point::new(0.20502900391817092, -1.495),
    ///     Point::new(0.0, -1.0),
    ///     Point::new(-0.002071000039577484, -1.005),
    ///     Point::new(-0.2071000039577484, -1.5),
    ///     Point::new(-0.2121000036597252, -1.4979289996623992),
    ///     Point::new(-0.707099974155426, -1.2928999662399292),
    ///     Point::new(-0.7100289744138718, -1.2916869664192199),
    ///     Point::new(-1.0, -1.1715999841690063),
    ///     Point::new(-1.0029289996623993, -1.1728129839897157),
    ///     Point::new(-1.2928999662399292, -1.2928999662399292),
    ///     Point::new(-1.1728129839897157, -1.0029289996623993),
    ///     Point::new(-1.1715999841690063, -1.0),
    ///     Point::new(-1.1728129839897157, -0.9970709997415542),
    ///     Point::new(-1.2928999662399292, -0.707099974155426),
    ///     Point::new(-1.29497096657753, -0.7020999744534493),
    ///     Point::new(-1.5, -0.2071000039577484),
    ///     Point::new(-1.005, -0.002071000039577484),
    ///     Point::new(-1.0, 0.0),
    ///     Point::new(-1.005, 0.002071000039577484)
    ///         ]);
    /// let simple_polygon = complex_polygon.simplify();
    /// assert!(simple_polygon.points.len() == 24);
    /// ```
    pub fn simplify(&self) -> Polygon {
        //remove all small triangles
        //when looping on 3 consecutive points
        let triangle_area = |(p1, p2, p3): (&Point, &Point, &Point)| {
            (p1.x * p2.y - p1.y * p2.x + p2.x * p3.y - p2.y * p3.x + p3.x * p1.y - p3.y * p1.x)
                .abs() / 2.0
        };
        let new_points: Vec<Point> = izip!(
            self.points.iter(),
            self.points.iter().cycle().skip(1),
            self.points.iter().cycle().skip(2)
        ).filter_map(|points| {
            if triangle_area(points) < 0.000001 {
                None
            } else {
                Some(points.1)
            }
        })
            .cloned()
            .collect();

        //now remove aligned points
        let final_points: Vec<Point> = izip!(
            new_points.iter(),
            new_points.iter().cycle().skip(1),
            new_points.iter().cycle().skip(2)
        ).filter_map(|(p1, p2, p3)| {
            if p1.is_aligned_with(p2, p3) {
                None
            } else {
                Some(*p2)
            }
        })
            .collect();

        assert!(final_points.len() > 2);
        Polygon::new(final_points)
    }
}
