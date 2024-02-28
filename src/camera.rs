use std::{cell::RefCell, time::Instant};

use glam::{vec3, Vec3};
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressIterator, ProgressStyle};
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::{
    hittable::Hittable,
    interval::interval,
    ray::{ray, Ray},
};

/// Converts a [`glam::Vec3`] scaled from `0.0`-`1.0` to an [`image::Rgb`] with values from `0`-`255`
fn vec_to_color(vec: Vec3, samples_per_pixel: u32) -> image::Rgb<u8> {
    // Antialiasing
    let scaled = vec / samples_per_pixel as f32;
    let intensity = interval(0.000, 0.999);

    image::Rgb([
        (256. * intensity.clamp(scaled.x)) as u8,
        (256. * intensity.clamp(scaled.y)) as u8,
        (256. * intensity.clamp(scaled.z)) as u8,
    ])
}

pub struct Camera {
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,

    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    rng: RefCell<ThreadRng>,
}

impl Camera {
    pub fn new(image_width: u32, image_height: u32, samples_per_pixel: u32) -> Self {
        let aspect_ratio = image_width as f32 / image_height as f32;

        // Determine viewport dimensions
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * aspect_ratio;

        let focal_length = 1.0;
        let camera_center = vec3(0., 0., 0.);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = vec3(viewport_width, 0., 0.);
        let viewport_v = vec3(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            camera_center - vec3(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            samples_per_pixel,

            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            rng: thread_rng().into(),
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        let out_file = std::env::args().nth(1).unwrap_or("output.png".to_owned());

        let start = Instant::now();
        let style = ProgressStyle::with_template(
        "RENDERING {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len} scanlines, ETA {eta})",
    )
    .unwrap();

        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        for y in (0..self.image_height).progress_with_style(style) {
            for x in 0..self.image_width {
                let mut color = Vec3::ZERO;

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color += self.ray_color(world, &ray);
                }

                img.put_pixel(x, y, vec_to_color(color, self.samples_per_pixel));
            }
        }

        println!("Finished in {:?}", start.elapsed());
        img.save(out_file).unwrap();
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (x as f32 * self.pixel_delta_u) + (y as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - ray_origin;

        ray(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px: f32 = -0.5 + self.rng.borrow_mut().gen::<f32>();
        let py: f32 = -0.5 + self.rng.borrow_mut().gen::<f32>();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn ray_color(&self, world: &dyn Hittable, ray: &Ray) -> Vec3 {
        if let Some(hit) = world.hit(ray, interval(0., f32::INFINITY)) {
            return 0.5 * (hit.normal + vec3(1., 1., 1.));
        }

        // Sky
        let unit_dir = ray.direction.normalize();
        let a = 0.5 * (unit_dir.y + 1.);

        (1. - a) * vec3(1., 1., 1.) + a * vec3(0.5, 0.7, 1.0)
    }
}
