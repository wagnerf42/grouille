//! This module allows for the initial classification
//! of polygons into a polygon tree.
use std::iter::repeat;
use Polygon;

/// Take some polygons to classify and return two vectors : classified ones and roots
pub fn brute_force_classification<'a>(
    polygons: &'a [Polygon],
) -> (Vec<(usize, usize)>, Vec<usize>) {
    // our results
    let mut roots: Vec<usize> = Vec::with_capacity(polygons.len()); // indices of all root polygons
    let mut classified: Vec<(usize, usize)> = Vec::with_capacity(polygons.len()); // (id, father_id)

    // to each polygon its father, self if none, -1 if not known yet
    let mut fathers: Vec<isize> = repeat(-1).take(polygons.len()).collect();

    while roots.len() + classified.len() != polygons.len() {
        // take a y with at least one not yet classified polygon
        let first_unclassified = (0..polygons.len()).find(|&i| fathers[i] == -1).unwrap();
        let points = polygons[first_unclassified].points();
        let y = (points[0].y + points[1].y) / 2.0;
        // use this y to figure out all polygons relative positions here
        let mut intersections = Vec::with_capacity(polygons.len() * 20);
        for (index, polygon) in polygons.iter().enumerate() {
            for x in polygon.intersections_at_y(y) {
                intersections.push((x, index));
            }
        }
        intersections.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        // now we loop on all intersections, figuring out who is included in whom
        let mut i = intersections.into_iter();
        let (_, mut previous_one) = i.next().unwrap();
        if fathers[previous_one] == -1 {
            roots.push(previous_one);
            fathers[previous_one] = previous_one as isize;
        }

        let mut counts = repeat(0).take(polygons.len()).collect::<Vec<usize>>();
        counts[previous_one] = 1;

        for (_, index) in i {
            if fathers[index] == -1 {
                // first time we see it, classify it
                if counts[previous_one] % 2 == 1 {
                    // we are included in neighbour : we are his son
                    fathers[index] = previous_one as isize;
                    classified.push((index, fathers[index] as usize));
                } else {
                    // we are not included in neighbour : we are his brother
                    if fathers[previous_one] as usize != previous_one {
                        fathers[index] = fathers[previous_one];
                        classified.push((index, fathers[index] as usize));
                    } else {
                        fathers[index] = index as isize;
                        roots.push(index);
                    }
                }
            }
            counts[index] += 1;
            previous_one = index;
        }
    }
    (classified, roots)
}
