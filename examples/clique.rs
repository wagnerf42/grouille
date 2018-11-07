extern crate grouille;
extern crate itertools;
extern crate rand;
use grouille::{tycat, Point};
use itertools::repeat_call;
use rand::random;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

type PointIndex = usize;

/// events for sweeping line algorithm.
enum Event {
    /// a point enters the system.
    Point(Point, PointIndex),
    /// a point tries to become dominant (at given point).
    Dominance(Point, Point, PointIndex),
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        // note : no more than one event of each type for each point
        match self {
            Event::Point(p, _) => match other {
                Event::Point(p2, _) => p.eq(p2),
                _ => false,
            },
            Event::Dominance(p, _, _) => match other {
                Event::Dominance(p2, _, _) => p.eq(p2),
                _ => false,
            },
        }
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let p1 = match self {
            Event::Point(p, _) => p,
            Event::Dominance(_, p, _) => p,
        };
        let p2 = match other {
            Event::Point(p, _) => p,
            Event::Dominance(_, p, _) => p,
        };
        p1.partial_cmp(p2)
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn compute_dominating_points<I: Iterator<Item = Point>>(
    points: I,
    dominating_points: &mut HashSet<PointIndex>,
) {
    let mut events: BinaryHeap<Event> = points
        .enumerate()
        .map(|(i, p)| Event::Point(p.clone(), i))
        .collect();
    let starting_event = events.pop().unwrap();
    let (mut dominating_point, point_index) = match starting_event {
        Event::Point(p, i) => (p, i),
        _ => panic!("no way"),
    };
    dominating_points.insert(point_index);

    while let Some(event) = events.pop() {
        match event {
            Event::Point(p, point_index) => {
                // compare point to currently dominating point
                let distance = (dominating_point.y - p.y) - (dominating_point.x - p.x);
                // at this x, p will become (maybe) dominant over dominating_point
                let dominance_x = p.x - distance;
                if dominance_x >= p.x {
                    dominating_point = p;
                    dominating_points.insert(point_index);
                } else {
                    events.push(Event::Dominance(
                        p,
                        Point::new(dominance_x, std::f64::INFINITY),
                        point_index,
                    ));
                }
            }
            Event::Dominance(point_becoming_dominant, _, point_index) => {
                // ok, some guy tries to become dominant. check if we allow it
                if point_becoming_dominant.x < dominating_point.x {
                    dominating_point = point_becoming_dominant;
                    dominating_points.insert(point_index);
                }
            }
        }
    }
}

fn main() {
    let points: Vec<Point> = repeat_call(|| Point::new(random(), random()))
        .take(10000)
        .collect();
    let mut dominating_points = HashSet::new();
    compute_dominating_points(points.iter().cloned(), &mut dominating_points);
    compute_dominating_points(
        points.iter().map(|p| Point::new(p.x, -p.y)),
        &mut dominating_points,
    );
    compute_dominating_points(
        points.iter().map(|p| Point::new(p.y, p.x)),
        &mut dominating_points,
    );
    compute_dominating_points(
        points.iter().map(|p| Point::new(p.y, -p.x)),
        &mut dominating_points,
    );
    let dpoints: Vec<_> = dominating_points.iter().map(|i| points[*i]).collect();
    tycat!(points, dpoints);
    println!(
        "we now have {} points instead of {} !",
        dpoints.len(),
        points.len()
    );
}
