#![forbid(unsafe_code)]
#![deny(clippy::all)]
pub mod hittable;
pub mod ray;

use std::{f32::INFINITY, rc::Rc, time::Instant};

use glam::{vec3, Vec3};
use hittable::Hittable;
use image::{ImageBuffer, RgbImage};

use indicatif::{ProgressIterator, ProgressStyle};
use ray::{ray, Ray};

use crate::hittable::{group::HittableGroup, sphere::Sphere};

/// Converts a [`glam::Vec3`] scaled from `0.0`-`1.0` to an [`image::Rgb`] with values from `0`-`255`
fn vec_to_color(vec: Vec3) -> image::Rgb<u8> {
    let scaled = 255. * vec;
    image::Rgb([scaled.x as u8, scaled.y as u8, scaled.z as u8])
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0., INFINITY) {
        return 0.5 * (hit.normal + vec3(1., 1., 1.));
    }

    // Sky
    let unit_dir = ray.direction.normalize();
    let a = 0.5 * (unit_dir.y + 1.);

    (1. - a) * vec3(1., 1., 1.) + a * vec3(0.5, 0.7, 1.0)
}

fn main() {
    let out_file = std::env::args().nth(1).unwrap_or("output.png".to_owned());

    // Image

    let image_width: u32 = 256;
    let image_height: u32 = 256;
    let aspect_ratio: f32 = image_width as f32 / image_height as f32;

    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * aspect_ratio;

    // World

    let mut world = HittableGroup::new();

    world.add(Rc::new(Sphere::new(vec3(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(vec3(0., -100.5, -1.), 100.)));

    // Camera

    let focal_length = 1.0;
    let camera_center = vec3(0., 0., 0.);

    let viewport_u = vec3(viewport_width, 0., 0.);
    let viewport_v = vec3(0., -viewport_height, 0.);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left =
        camera_center - vec3(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Rendering

    let start = Instant::now();
    let style = ProgressStyle::with_template(
        "RENDERING {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len} scanlines, ETA {eta})",
    )
    .unwrap();

    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    for y in (0..image_height).progress_with_style(style) {
        for x in 0..image_width {
            let pixel_center =
                pixel00_loc + (x as f32 * pixel_delta_u) + (y as f32 * pixel_delta_v);
            let ray_dir = pixel_center - camera_center;

            let ray = ray(camera_center, ray_dir);
            let color = ray_color(&ray, &world);

            img.put_pixel(x, y, vec_to_color(color));
        }
    }

    println!("Finished in {:?}", start.elapsed());
    img.save(out_file).unwrap();
}
