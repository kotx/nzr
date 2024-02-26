use std::rc::Rc;

use crate::interval::{interval, Interval};

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
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<super::HitRecord> {
        let mut closest_hit = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, interval(ray_t.min, closest_so_far)) {
                closest_hit = Some(rec);
                closest_so_far = rec.t;
            }
        }

        closest_hit
    }
}
