//! We the use the 'tycat' command from the 'terminology' unix terminal to display
//! various graphical objects interactively on the console.
//! We define here the `Tycat` trait which all shapes which can be displayed
//! graphically need to implement.
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::once;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use {Point, Polygon, Quadrant, Segment};

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
        let middle = self.start.center_with(&self.end);
        let angle = (self.end - self.start).angle();
        format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\
             <use xlink:href=\"#a\" x=\"{}\" y=\"{}\" transform=\"rotate({} {} {})\"/>",
            self.start.x,
            self.start.y,
            self.end.x,
            self.end.y,
            middle.x,
            middle.y,
            angle.to_degrees(),
            middle.x,
            middle.y
        )
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

impl Tycat for Polygon {
    fn quadrant(&self) -> Quadrant {
        self.points.iter().fold(Quadrant::new(), |q, p| q.add(&p))
    }
    fn svg_string(&self) -> String {
        once("<polygon points=\"".to_string())
            .chain(self.points.iter().map(|p| format!(" {},{}", p.x, p.y)))
            .chain(once("\" opacity=\"0.5\"/>".to_string()))
            .collect()
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
        xmin, ymin, width, height
    )?;
    svg_file.write_all(b"xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n")?;

    // white background
    write!(svg_file, "<rect x=\"{}\" y=\"{}\" ", xmin, ymin)?;
    write!(
        svg_file,
        "width=\"{}\" height=\"{}\" fill=\"white\"/>\n",
        width, height
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
