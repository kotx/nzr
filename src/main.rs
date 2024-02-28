#![forbid(unsafe_code)]
#![deny(clippy::all)]
pub mod camera;
pub mod hittable;
pub mod interval;
pub mod ray;
pub mod util;

use std::rc::Rc;

use camera::Camera;
use glam::vec3;

use crate::hittable::{group::HittableGroup, sphere::Sphere};

fn main() {
    let mut world = HittableGroup::new();

    world.add(Rc::new(Sphere::new(vec3(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(vec3(0., -100.5, -1.), 100.)));

    let cam = Camera::new(256, 256, 100);
    cam.render(&world);
}
