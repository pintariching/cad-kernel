use super::{BoundaryArc, BoundaryLine, BoundaryPolygon};

pub struct BoundaryLoop {
    elements: Vec<BoundaryElement>,
}

pub enum BoundaryElement {
    BoundaryLine(BoundaryLine),
    BoundaryPolygon(BoundaryPolygon),
    BoundaryArc(BoundaryArc),
}
