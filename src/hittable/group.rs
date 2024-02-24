use std::rc::Rc;

use super::Hittable;

pub struct HittableGroup {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableGroup {
    pub fn new() -> Self {
        HittableGroup {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableGroup {
    fn hit(&self, ray: &crate::ray::Ray, ray_tmin: f32, ray_tmax: f32) -> Option<super::HitRecord> {
        let mut closest_hit = None;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, ray_tmin, closest_so_far) {
                closest_hit = Some(rec);
                closest_so_far = rec.t;
            }
        }

        closest_hit
    }
}
