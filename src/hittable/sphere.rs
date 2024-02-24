use glam::Vec3;

use super::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            None
        } else {
            let sqrtd = discriminant.sqrt();

            let mut root = (-half_b - sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                root = (-half_b + sqrtd) / a;
                if root <= ray_tmin || root >= ray_tmax {
                    return None;
                }
            }

            let t = root;
            let point = ray.at(t);
            let outward_normal = (point - self.center) / self.radius;

            let mut rec = HitRecord { t, point, normal: Vec3::ZERO, front_face: false };
            rec.set_face_normal(ray, outward_normal); // set normal and front_face

            Some(rec)
        }
    }
}
