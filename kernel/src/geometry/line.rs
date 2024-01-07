use glam::Vec3;

use crate::point::Point;
use crate::Plane;

#[derive(Debug)]
pub enum Line {
    /// A line can be represented by a parametric equation in the form of `P(t)=P_0+t⋅v⃗`
    /// where `P_0` is a point on the line, `v⃗` is the direction vector of the line, and `t` is a
    /// parameter that varies over the real numbers. This representation allows infinite
    /// points on the line to be calculated by varying `t`.
    Parametric(ParametricLine),

    // Representing a line by two distinct points `A` and `B` lying on the line.
    TwoPoint(TwoPointLine),

    /// A line in 3D space can also be represented using an implicit equation of the form
    /// `Ax+By+Cz+D=0`, where A,B,C are the direction coefficients and D
    /// is a constant term defining the line.
    Implicit(ImplicitLine),
}

impl Line {
    pub fn to_parametric(&self) -> &ParametricLine {
        match self {
            Line::Parametric(p) => p,
            Line::TwoPoint(_) => todo!(),
            Line::Implicit(_) => todo!(),
        }
    }

    pub fn to_two_point_line(&self) -> &TwoPointLine {
        match self {
            Line::TwoPoint(l) => l,
            Line::Parametric(_) => todo!(),
            Line::Implicit(_) => todo!(),
        }
    }

    pub fn project_to_plane(&self, plane: &Plane) -> Self {
        let tpl = self.to_two_point_line();

        let a = tpl.a.project_to_plane(plane);
        let b = tpl.b.project_to_plane(plane);

        Self::TwoPoint(TwoPointLine::new(a.0, b.0))
    }

    pub fn generate_projected_quad(&self, plane: &Plane, width: f32) -> [Vec3; 6] {
        let projected_line = self.project_to_plane(plane);
        let tpl = projected_line.to_two_point_line();

        let line_dir = (tpl.b.0 - tpl.a.0).normalize();
        let line_up = line_dir.cross(plane.normal);

        let half_width = width / 2.;

        let tl = tpl.a.0 + line_up * half_width;
        let bl = tpl.a.0 - line_up * half_width;

        let tr = tpl.b.0 + line_up * half_width;
        let br = tpl.b.0 - line_up * half_width;

        [bl, tr, tl, bl, br, tr]
    }
}

#[derive(Debug)]
pub struct ParametricLine {
    pub p: Vec3,
    pub v: Vec3,
}

impl ParametricLine {
    pub fn new(p: Vec3, v: Vec3) -> Self {
        Self { p, v }
    }
}

#[derive(Debug)]
pub struct TwoPointLine {
    pub a: Point,
    pub b: Point,
}

impl TwoPointLine {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            a: Point(a),
            b: Point(b),
        }
    }

    pub fn normal(&self) -> Vec3 {
        (self.b.0 - self.a.0).normalize()
    }

    pub fn to_points(&self) -> [[f32; 3]; 2] {
        [self.a.to_array(), self.b.to_array()]
    }
}

#[derive(Debug)]
pub struct ImplicitLine {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}
