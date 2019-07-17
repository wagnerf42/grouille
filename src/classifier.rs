//! This module allows for the initial classification
//! of polygons into a polygon tree.
use crate::shape::Shape;
use std::iter::repeat;

/// Take some polygons to classify and return three vectors : classified ones, roots, fathers
pub fn brute_force_classification<'a, S: Shape>(
    shapes: &'a [S],
) -> (Vec<(usize, usize)>, Vec<usize>, Vec<isize>) {
    // our results
    let mut roots: Vec<usize> = Vec::with_capacity(shapes.len()); // indices of all root shapes
    let mut classified: Vec<(usize, usize)> = Vec::with_capacity(shapes.len()); // (id, father_id)

    // to each polygon its father, self if none, -1 if not known yet
    let mut fathers: Vec<isize> = repeat(-1).take(shapes.len()).collect();

    while roots.len() + classified.len() != shapes.len() {
        // take a y with at least one not yet classified polygon
        let first_unclassified = (0..shapes.len()).find(|&i| fathers[i] == -1).unwrap();
        let y = shapes[first_unclassified].inner_y();
        // use this y to figure out all polygons relative positions here
        let mut intersections = Vec::with_capacity(shapes.len() * 20);
        for (index, shape) in shapes.iter().enumerate() {
            let (shape_ymin, shape_ymax) = shape.quadrant().limits(1);
            if y >= shape_ymin && y <= shape_ymax {
                shape.register_intersections(&mut intersections, index, y);
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

        let mut counts = repeat(0).take(shapes.len()).collect::<Vec<usize>>();
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
    (classified, roots, fathers)
}
