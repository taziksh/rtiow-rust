use crate::hittable::{Hittable, HitRecord};
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Self {
            center: center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = self.center - ray.origin();

        let dir: Vec3 = ray.direction();
        let a = dir.dot(&dir);
        let b = -2.0 * dir.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;

        // discriminant
        let d = b * b - 4.0 * a * c;

        if d < 0.0 {
            return None;
        }

        let sqrtd = d.sqrt();
        
        // Case 1: Intersection is behind the ray (-b - ...)
        let mut root = (-b - sqrtd) / (2.0 * a);
        if root < t_min || root > t_max {
            // Case 2: Intersection is ahead of ray (-b + ...)
            root = (-b + sqrtd) / (2.0 * a);
            if root < t_min || root > t_max {
                return None;
            }
        }

        // make hitrecord
        let t = root;
        let point = ray.at(t);

        // normal vectors are unit vectors
        let outward_normal = (point - self.center) / self.radius;
        
        let mut hit_record = HitRecord {
            point,
            t,
            normal: outward_normal,
            front_face: false
        };

        hit_record.set_face_normal(ray, outward_normal);

        return Some(hit_record);
    }

}