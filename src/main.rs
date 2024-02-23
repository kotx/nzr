#![forbid(unsafe_code)]
#![deny(clippy::all)]
pub mod ray;

use glam::{vec3, Vec3};
use image::{ImageBuffer, RgbImage};
use ray::{ray, Ray};

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
const ASPECT_RATIO: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;

const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;

/// Converts a [`glam::Vec3`] scaled from `0.0`-`1.0` to an [`image::Rgb`] with values from `0`-`255`
fn vec_to_color(vec: Vec3) -> image::Rgb<u8> {
    let scaled = 255. * vec;
    image::Rgb([scaled.x as u8, scaled.y as u8, scaled.z as u8])
}

fn ray_color(ray: Ray) -> image::Rgb<u8> {
    let unit_dir = ray.direction.normalize();
    let a = 0.5 * (unit_dir.y + 1.);

    let color = (1. - a) * vec3(1., 1., 1.) + a * vec3(0.5, 0.7, 1.0);
    vec_to_color(color)
}

fn main() {
    let focal_length = 1.0;
    let camera_center = vec3(0., 0., 0.);

    let viewport_u = vec3(VIEWPORT_WIDTH, 0., 0.);
    let viewport_v = vec3(0., -VIEWPORT_HEIGHT, 0.);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f32;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f32;

    let viewport_upper_left =
        camera_center - vec3(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (x as f32 * pixel_delta_u) + (y as f32 * pixel_delta_v);
            let ray_dir = pixel_center - camera_center;

            let ray = ray(pixel_center, ray_dir);
            let color = ray_color(ray);

            img.put_pixel(x, y, color);
        }
    }

    img.save("output.png").unwrap();
}
