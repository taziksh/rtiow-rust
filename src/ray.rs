use crate::vec3::Vec3;
use crate::point3::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray {orig, dir}
    }

    pub fn origin(&self) -> Point3 { self.orig }
    pub fn direction(&self) -> Vec3 { self.dir }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}