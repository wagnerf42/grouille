//! We the use the 'tycat' command from the 'terminology' unix terminal to display
//! various graphical objects interactively on the console.
//! We define here the `Tycat` trait which all shapes which can be displayed
//! graphically need to implement.
use std::f64::consts::{FRAC_PI_2, PI};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::once;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use {Arc, ElementaryPath, HoledPolygon, Pocket, Point, Polygon, Quadrant, Segment, Vector};

/// Anything displayable in terminology needs to implement this trait.
pub trait Tycat {
    /// Return smallest quadrant containing us.
    fn quadrant(&self) -> Quadrant;
    /// Return svg string displaying us.
    fn svg_string(&self) -> String;
}

impl Tycat for Point {
    fn quadrant(&self) -> Quadrant {
        Quadrant::new().add(self)
    }
    fn svg_string(&self) -> String {
        format!("<use xlink:href=\"#c\" x=\"{}\" y=\"{}\"/>", self.x, self.y)
    }
}

impl Tycat for Segment {
    fn quadrant(&self) -> Quadrant {
        Quadrant::new().add(&self.start).add(&self.end)
    }
    fn svg_string(&self) -> String {
        // We display a nice arrow at midpoint
        //let middle = self.start.center_with(&self.end);
        //let angle = (self.end - self.start).angle();
        format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
            self.start.x,
            self.start.y,
            self.end.x,
            self.end.y,
        )
        //        format!(
        //            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\
        //             <use xlink:href=\"#a\" x=\"{}\" y=\"{}\" transform=\"rotate({} {} {})\"/>",
        //            self.start.x,
        //            self.start.y,
        //            self.end.x,
        //            self.end.y,
        //            middle.x,
        //            middle.y,
        //            angle.to_degrees(),
        //            middle.x,
        //            middle.y
        //        )
    }
}

impl<'a, T: Tycat> Tycat for &'a T {
    fn quadrant(&self) -> Quadrant {
        (*self).quadrant()
    }
    fn svg_string(&self) -> String {
        (*self).svg_string()
    }
}

impl<T: Tycat> Tycat for Vec<T> {
    fn quadrant(&self) -> Quadrant {
        self.iter()
            .map(|t| t.quadrant())
            .fold(Quadrant::new(), |mut acc, q| {
                acc.update(&q);
                acc
            })
    }
    fn svg_string(&self) -> String {
        self.iter().map(|t| t.svg_string()).collect()
    }
}

impl<'a, T: Tycat> Tycat for &'a [T] {
    fn quadrant(&self) -> Quadrant {
        self.iter()
            .map(|t| t.quadrant())
            .fold(Quadrant::new(), |mut acc, q| {
                acc.update(&q);
                acc
            })
    }
    fn svg_string(&self) -> String {
        self.iter().map(|t| t.svg_string()).collect()
    }
}

impl Tycat for Polygon {
    fn quadrant(&self) -> Quadrant {
        self.quadrant
    }
    fn svg_string(&self) -> String {
        once("<polygon points=\"".to_string())
            .chain(self.points().iter().map(|p| format!(" {},{}", p.x, p.y)))
            .chain(once("\" opacity=\"0.5\"/>".to_string()))
            .collect()
    }
}

/// Turn polygon into svg path.
fn polygon_path<'a>(polygon: &'a Polygon) -> impl Iterator<Item = String> + 'a {
    let mut points = polygon.points().iter().map(|p| format!("{} {} ", p.x, p.y));
    let start = points.next().unwrap();
    once(format!("M {} L ", start)).chain(points)
}

impl Tycat for HoledPolygon {
    fn quadrant(&self) -> Quadrant {
        self.outer_polygon.quadrant
    }
    fn svg_string(&self) -> String {
        once("<path d=\"".to_string())
            .chain(
                once(&self.outer_polygon) // clockwise
                    .chain(self.holes.iter()) // counter clockwise
                    .flat_map(|p| polygon_path(p)),
            )
            .chain(once("\" />".to_string()))
            .collect()
    }
}

impl Tycat for Arc {
    fn quadrant(&self) -> Quadrant {
        Quadrant {
            mins: [self.center.x - self.radius, self.center.y - self.radius],
            maxs: [self.center.x + self.radius, self.center.y + self.radius],
        }
    }
    fn svg_string(&self) -> String {
        let center_string = self.center.svg_string();
        // go always for the small arc
        let sweep_flag = if self.angle() > PI { 1 } else { 0 };
        let arc_string = format!(
            "<path d=\"M{},{} A{},{} 0 0,{} {},{}\" fill=\"none\"/>",
            self.start.x,
            self.start.y,
            self.radius,
            self.radius,
            sweep_flag,
            self.end.x,
            self.end.y
        );
        // now draw a small arrow indicating orientation
        let mut middle_angle =
            ((self.start - self.center).angle() + (self.end - self.center).angle()) / 2.0;
        // we need to figure out where is the middle point between two candidates
        let possible_points = [
            self.center + Vector::polar(self.radius, middle_angle),
            self.center + Vector::polar(self.radius, middle_angle + PI),
        ];
        let distances = [
            self.start.distance_to(&possible_points[0]),
            self.start.distance_to(&possible_points[1]),
        ];
        let tangent_point = if distances[0] < distances[1] {
            possible_points[0]
        } else {
            middle_angle += PI;
            possible_points[1]
        };
        let tangent_angle = if sweep_flag == 1 {
            middle_angle + FRAC_PI_2
        } else {
            middle_angle - FRAC_PI_2
        };
        let arrow_string = format!(
            "<use xlink:href=\"#a\" x=\"{}\" y=\"{}\" transform=\"rotate({} {} {})\"/>",
            tangent_point.x,
            tangent_point.y,
            tangent_angle * 180.0 / PI,
            tangent_point.x,
            tangent_point.y
        );
        center_string + &arc_string + &arrow_string
    }
}

impl Tycat for ElementaryPath {
    fn quadrant(&self) -> Quadrant {
        match *self {
            ElementaryPath::Arc(ref a) => a.quadrant(),
            ElementaryPath::Segment(ref s) => s.quadrant(),
        }
    }
    fn svg_string(&self) -> String {
        match *self {
            ElementaryPath::Arc(ref a) => a.svg_string(),
            ElementaryPath::Segment(ref s) => s.svg_string(),
        }
    }
}

impl Tycat for Pocket {
    fn quadrant(&self) -> Quadrant {
        self.quadrant
    }
    fn svg_string(&self) -> String {
        if let Some(first_path) = self.edge.first() {
            let starting_point = first_path.start();
            once(format!(
                "<path d=\"M{},{}",
                starting_point.x,
                starting_point.y
            )).chain(self.edge.iter().map(|p| match *p {
                ElementaryPath::Segment(ref s) => format!(" L {} {}", s.end.x, s.end.y),
                ElementaryPath::Arc(ref a) => {
                    let sweep_flag = if a.angle() > PI { 1 } else { 0 };
                    format!(
                        " A {},{} 0 0,{} {},{}",
                        a.radius,
                        a.radius,
                        sweep_flag,
                        a.end.x,
                        a.end.y
                    )
                }
            }))
                .chain(once("\"/>".to_string()))
                .collect()
        } else {
            String::new()
        }
    }
}

/* below this point is all the dirty svg intrinsics */

static FILE_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

/// Common colors
pub const SVG_COLORS: [&str; 35] = [
    "red",
    "green",
    "blue",
    "purple",
    "orange",
    "saddlebrown",
    "mediumseagreen",
    "darkolivegreen",
    "lightskyblue",
    "dimgray",
    "mediumpurple",
    "midnightblue",
    "chartreuse",
    "darkorchid",
    "hotpink",
    "peru",
    "goldenrod",
    "mediumslateblue",
    "orangered",
    "darkmagenta",
    "darkgoldenrod",
    "mediumslateblue",
    "firebrick",
    "palegreen",
    "royalblue",
    "tan",
    "tomato",
    "springgreen",
    "pink",
    "orchid",
    "saddlebrown",
    "moccasin",
    "mistyrose",
    "cornflowerblue",
    "darkgrey",
];

/// Do a tycat in terminology be resizing svg strings according to given quadrant.
/// This function should not be called directly but rather throught the tycat! macro.
pub fn display(quadrant: &Quadrant, svg_strings: &[String]) -> io::Result<()> {
    let file_number = FILE_COUNT.fetch_add(1, Ordering::SeqCst);
    let filename = format!("/tmp/test-{:04}.svg", file_number);
    println!("[{}]", file_number);
    let mut svg_file = File::create(&filename)?;

    // write header
    svg_file.write_all(b"<svg width=\"640\" height=\"480\" ")?;
    let (xmin, xmax) = quadrant.limits(0);
    let (ymin, ymax) = quadrant.limits(1);
    let width = xmax - xmin;
    let height = ymax - ymin;
    write!(
        svg_file,
        "viewBox=\"{} {} {} {}\" ",
        xmin,
        ymin,
        width,
        height
    )?;
    svg_file.write_all(b"xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n")?;

    // white background
    write!(svg_file, "<rect x=\"{}\" y=\"{}\" ", xmin, ymin)?;
    write!(
        svg_file,
        "width=\"{}\" height=\"{}\" fill=\"white\"/>\n",
        width,
        height
    )?;

    // circle definition and stroke size
    let xscale = 640.0 / width;
    let yscale = 480.0 / height;
    let scale = if xscale < yscale { xscale } else { yscale };
    let stroke = 3.0 / scale;
    write!(svg_file, "<defs>\n")?;
    write!(
        svg_file,
        "<symbol id=\"c\"><circle r=\"{}\"/></symbol>\n",
        2.0 * stroke
    )?;
    write!(
        svg_file,
        //"<symbol id=\"a\"></symbol>\n", // this removes segments orientation
        "<symbol id=\"a\"><line x1=\"{}\" y1=\"{}\" x2=\"0.0\" y2=\"0.0\"/>\
         <line x1=\"{}\" y1=\"{}\" x2=\"0.0\" y2=\"0.0\"/></symbol>\n",
        -3.0 * stroke,
        -3.0 * stroke,
        -3.0 * stroke,
        3.0 * stroke,
    )?;
    write!(svg_file, "</defs>\n")?;
    write!(
        svg_file,
        "<g stroke-width=\"{}\" opacity=\"0.7\">\n",
        stroke
    )?;

    for svg_string in svg_strings {
        svg_file.write_all(svg_string.as_bytes())?;
    }
    svg_file.write_all(b"</g></svg>")?;
    Command::new("tycat").arg(filename).status()?;
    Ok(())
}

#[macro_export]
macro_rules! tycat {
    ( $($x:expr ), +) => {
        {
            use $crate::Quadrant;
            use $crate::tycat::{display, Tycat};
            let mut quadrant = Quadrant::new();
            let mut svg_strings = Vec::new();
            let mut colors = $crate::tycat::SVG_COLORS.iter().cycle();
            $(
                quadrant.update(&$x.quadrant());
                svg_strings.push({
                    let color = colors.next().unwrap();
                    format!("<g fill=\"{}\" stroke=\"{}\">{}</g>\n", color, color, $x.svg_string())
                });
             )*
                display(&quadrant, &svg_strings).expect("tycat failed");
        }
    }
}
