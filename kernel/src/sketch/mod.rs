mod relations;

use crate::line::Line;
use crate::{Arc, Plane, Point};

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
    plane: SketchPlane,
    elements: Vec<SketchElement>,
}
