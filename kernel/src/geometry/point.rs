use std::ops::Deref;

use glam::Vec3;

use crate::Plane;

#[derive(Debug)]
pub struct Point(pub Vec3);

impl Deref for Point {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Point {
    pub fn new(p: Vec3) -> Self {
        Self(p)
    }

    pub fn project_to_plane(&self, plane: &Plane) -> Self {
        let v = self.0 - plane.center;

        let dist = v.dot(plane.normal);

        Point(self.0 - dist * plane.normal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_point_onto_plane() {
        let plane = Plane::new(Vec3::Y, Vec3::ZERO);
        let point = Point::new(Vec3::new(5., 7., 2.));

        let projected = point.project_to_plane(&plane);

        assert_eq!(projected.0, Vec3::new(5., 0., 2.));

        let plane = Plane::new(Vec3::new(0., 1., 1.), Vec3::ZERO);

        let projected = point.project_to_plane(&plane);

        assert_eq!(projected.0, Vec3::new(5., 2.5, -2.5));
    }
}
