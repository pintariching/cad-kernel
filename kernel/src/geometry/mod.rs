use glam::Vec3;

pub mod line;
pub mod point;

pub struct Plane {
    pub normal: Vec3,
    pub center: Vec3,
}

impl Plane {
    pub const XY: Self = Self {
        normal: Vec3::Z,
        center: Vec3::ZERO,
    };
    pub const XZ: Self = Self {
        normal: Vec3::Y,
        center: Vec3::ZERO,
    };
    pub const YZ: Self = Self {
        normal: Vec3::X,
        center: Vec3::ZERO,
    };

    pub fn new(normal: Vec3, center: Vec3) -> Self {
        Self {
            normal: normal.normalize(),
            center,
        }
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
