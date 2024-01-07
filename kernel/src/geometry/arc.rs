use glam::{Mat4, Vec3};

use crate::line::{Line, TwoPointLine};
use crate::PolyLine;

pub struct Arc {
    pub radius: f32,
    pub start: Vec3,
    pub end: Vec3,
    pub center: Vec3,
    pub direction: ArcDirection,
}

pub enum ArcDirection {
    CW,
    CCW,
}

impl Arc {
    // Adapted from http://slabode.exofire.net/circle_draw.shtml
    pub fn to_lines(&self, segments: u32) -> Vec<Line> {
        let start = self.start - self.center;
        let end = self.end - self.center;

        let axis = (start - self.center).cross(end - self.center).normalize();

        let s = match self.direction {
            ArcDirection::CW => segments as f32,
            ArcDirection::CCW => -(segments as f32),
        };

        let arc_angle = start.angle_between(end);

        let angle = arc_angle / s;

        let rotation_matrix = Mat4::from_axis_angle(axis, angle);

        let mut out = Vec::new();
        let mut p = self.start;

        for _ in 0..segments {
            let a = p;
            let b = rotation_matrix.transform_vector3(p);

            out.push(Line::TwoPoint(TwoPointLine::new(a, b)));

            p = b;
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_polyline() {
        let arc = Arc {
            radius: 5.,
            start: Vec3::new(5., 0., 0.),
            end: Vec3::new(0., 5., 0.),
            center: Vec3::new(0., 0., 0.),
            direction: ArcDirection::CW,
        };

        let polyline = arc.to_polyline(6);

        assert_eq!(polyline.lines.len(), 6);
    }
}
