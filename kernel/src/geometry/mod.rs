use glam::Vec3;

pub mod line;

pub struct Point(pub Vec3);

impl Point {
    pub fn new(p: Vec3) -> Self {
        Self(p)
    }
}

pub struct Plane {
    normal: Vec3,
    center: Vec3,
}

impl Plane {
    pub const XY: Self = Self::new(Vec3::Z, Vec3::ZERO);
    pub const XZ: Self = Self::new(Vec3::Y, Vec3::ZERO);
    pub const YZ: Self = Self::new(Vec3::X, Vec3::ZERO);

    pub const fn new(normal: Vec3, center: Vec3) -> Self {
        Self { normal, center }
    }
}

pub struct Circle {
    center: Vec3,
    radius: f32,
}

impl Circle {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

pub struct Arc {
    radius: f32,
    start: Vec3,
    end: Vec3,
    direction: ArcDirection,
}

pub enum ArcDirection {
    CW,
    CCW,
}
