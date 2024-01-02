use glam::Vec3;

use crate::line::{Line, TwoPointLine};
use crate::point::Point;

pub struct BoundaryLine {
    line: Line,
    a: Point,
    b: Point,
}

impl BoundaryLine {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            line: Line::TwoPoint(TwoPointLine::new(a, b)),
            a: Point(a),
            b: Point(b),
        }
    }
}
