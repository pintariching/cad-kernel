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
}

#[derive(Debug)]
pub struct ImplicitLine {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}
