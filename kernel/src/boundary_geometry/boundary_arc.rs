use glam::Vec3;

use crate::point::Point;
use crate::Circle;

pub enum Direction {
    CW,
    CCW,
}

pub struct BoundaryArc {
    circle: Circle,
    start: Point,
    end: Point,
    direction: Direction,
}

impl BoundaryArc {
    pub fn new(center: Vec3, radius: f32, start: Vec3, end: Vec3, direction: Direction) -> Self {
        Self {
            circle: Circle::new(center, radius),
            start: Point(start),
            end: Point(end),
            direction,
        }
    }
}
