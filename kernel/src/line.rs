use glam::Vec3;

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

    pub fn to_two_point(&self) -> &TwoPointLine {
        match self {
            Line::TwoPoint(l) => l,
            Line::Parametric(_) => todo!(),
            Line::Implicit(_) => todo!(),
        }
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
    pub a: Vec3,
    pub b: Vec3,
}

impl TwoPointLine {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self { a, b }
    }

    pub fn normal(&self) -> Vec3 {
        (self.b - self.a).normalize()
    }

    pub fn project_to_plane(&self, plane_normal: Vec3, plane_point: Vec3) -> TwoPointLine {
        let w_a = plane_point - self.a;
        let p_a = w_a.dot(plane_normal) * plane_normal + self.a;

        let w_b = plane_point - self.b;
        let p_b = w_b.dot(plane_normal) * plane_normal + self.b;

        TwoPointLine { a: p_a, b: p_b }
    }
}

#[derive(Debug)]
pub struct ImplicitLine {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_two_point_line_to_plane() {
        let plane_normal = Vec3::new(0., 0., 1.);
        let plane_point = Vec3::new(0., 0., 0.);

        let line = TwoPointLine::new(Vec3::new(1., 1., 1.), Vec3::new(-1., -1., 1.));

        let projected_line = line.project_to_plane(plane_normal, plane_point);

        assert_eq!(projected_line.a, Vec3::new(1., 1., 0.));
        assert_eq!(projected_line.b, Vec3::new(-1., -1., 0.));
    }

    #[test]
    fn test_project_two_point_line_to_inclined_plane() {
        let plane_normal = Vec3::new(0., 0.5, 0.5).normalize();
        let plane_point = Vec3::new(0., 0., 0.);

        let line = TwoPointLine::new(Vec3::new(1., 1., 2.), Vec3::new(-1., -1., 2.));

        let projected_line = line.project_to_plane(plane_normal, plane_point);

        assert_eq!(projected_line.a, Vec3::new(1., -0.49999988, 0.5000001));
        assert_eq!(projected_line.b, Vec3::new(-1., -1.5, 1.5));
    }
}
