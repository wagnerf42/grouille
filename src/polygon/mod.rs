//! Polygon module.
use itertools::Itertools;
use std::iter::once;
use {Point, Quadrant};

pub mod polygon_builder;

/// Oriented polygons.
#[derive(Debug)]
pub struct Polygon {
    /// Vector of all points forming the edge of the polygon.
    points: Vec<Point>,
    /// Min sized quadrant containing us.
    pub quadrant: Quadrant,
}

/// Return area (SIGNED) of polygon delimited by given set of points.
/// pre-condition: at least 3 points
fn area(points: &[Point]) -> f64 {
    assert!(points.len() >= 3);
    points
        .windows(2)
        .chain(once(
            vec![
                points.last().cloned().unwrap(),
                points.first().cloned().unwrap(),
            ].as_slice(),
        ))
        .map(|p| p[0].x * p[1].y - p[0].y * p[1].x)
        .sum::<f64>() / 2.0
}

impl Polygon {
    /// Create polygon out of given points vector.
    pub fn new(points: Vec<Point>) -> Polygon {
        let poly = Polygon {
            points,
            quadrant: Quadrant::new(),
        };
        for point in &poly.points {
            poly.quadrant.add(point);
        }
        poly
    }

    /// Return a read-only iterator on our points.
    pub fn points<'a>(&'a self) -> impl Iterator<Item = &'a Point> + 'a {
        self.points.iter()
    }

    /// Returns area taken by polygon.
    /// Negative or Positive depending on orientation.
    pub fn area(&self) -> f64 {
        area(&self.points)
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
        let mut final_points: Vec<Point> = self
            .points
            .windows(3)
            .chain(once(
                vec![
                    self.points[self.points.len() - 2],
                    self.points.last().cloned().unwrap(),
                    self.points.first().cloned().unwrap(),
                ].as_slice(),
            ))
            .chain(once(
                vec![
                    self.points.last().cloned().unwrap(),
                    self.points[0],
                    self.points[1],
                ].as_slice(),
            ))
            .filter_map(|points| {
                if area(points).abs() > 0.00001 {
                    None
                } else {
                    Some(points[1])
                }
            })
            .tuple_windows()
            .filter_map(|(p1, p2, p3)| {
                if p1.is_aligned_with(&p2, &p3) {
                    None
                } else {
                    Some(p2)
                }
            })
            .collect();
        //now test last point for alignment
        if final_points[final_points.len() - 2]
            .is_aligned_with(final_points.last().unwrap(), final_points.first().unwrap())
        {
            final_points.pop();
        }

        assert!(final_points.len() > 2);
        Polygon::new(final_points)
    }
}
