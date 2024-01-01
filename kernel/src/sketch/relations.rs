use super::{SketchArc, SketchElement, SketchLine, SketchPoint};

pub enum Relation {
    Horizontal(SketchLine),
    Vertical(SketchLine),
    Coincident(Coincident),
    Perpendicular(Perpendicular),
    Tangent(),
    Fixed,
    Colinear,
    Coradial,
    Parallel,
    Concentric,
    Midpoint,
    Intersection,
    Equal,
}

pub struct Coincident {
    point: SketchPoint,
    other: CoincidentOther,
}

pub enum CoincidentOther {
    Line(SketchLine),
    Arc(SketchArc),
}

pub struct Perpendicular {
    line: SketchLine,
    perp: SketchLine,
}

pub struct Tangent {
    arc: SketchArc,
    other: TangentOther,
}

pub enum TangentOther {
    Line(SketchLine),
    Arc(SketchArc),
}
