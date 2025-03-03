use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::point3::Point3;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub t: f64,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // if the dot product is negative, the ray is inside the sphere
        self.front_face = Vec3::dot(&ray.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
