mod relations;

use crate::arc::Arc;
use crate::line::Line;
use crate::point::Point;
use crate::Plane;

pub struct SketchPlane(pub Plane);

impl SketchPlane {
    pub const XY: Self = Self(Plane::XY);
    pub const XZ: Self = Self(Plane::XZ);
    pub const YZ: Self = Self(Plane::YZ);
}

pub struct SketchLine(pub Line);

pub struct SketchPoint(pub Point);

pub struct SketchArc(pub Arc);

pub enum SketchElement {
    Line(SketchLine),
    Point(SketchPoint),
    Arc(SketchArc),
}

pub struct Sketch {
    pub plane: SketchPlane,
    pub elements: Vec<SketchElement>,
}

impl Sketch {
    pub fn to_lines(&self) -> Vec<Line> {
        let mut out = Vec::new();

        for element in &self.elements {
            match element {
                SketchElement::Arc(arc) => {
                    let mut lines = arc.0.to_lines(16);
                    out.append(&mut lines);
                }
                _ => (),
            }
        }

        out
    }
}
