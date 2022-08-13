pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

use std::f64::INFINITY;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use indicatif::ProgressBar;
use material::{lambertian::Lambertian, metal::Metal};
use rand::{Rng, RngCore};
use ray::Ray;
use vec3::Color;

use crate::{hittable::HittableList, sphere::Sphere, vec3::Point3};

fn ray_color<T: Hittable>(r: Ray, world: &T, depth: usize, rng: &mut Box<dyn RngCore>) -> Color {
    if depth <= 0 {
        return Color::new(0, 0, 0);
    }
    let mut rec: HitRecord = HitRecord::default();
    if world.hit(&r, 0.001, INFINITY, &mut rec) {
        let material = rec.material.clone();
        let (attenuation, scattered, flg) = material.scatter(&r, &mut rec, rng);
        if flg {
            return attenuation * ray_color(scattered, world, depth - 1, rng);
        }
        return Color::default();
    }
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (Color::new(1.0, 1.0, 1.0)) * (1.0 - t) + (Color::new(0.5, 0.7, 1.0) * t);
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

fn scale_color(color: Color, samples_per_pixel: u32) -> Color {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (color.x() * scale).sqrt();
    let g = (color.y() * scale).sqrt();
    let b = (color.z() * scale).sqrt();
    let n0999 = 1.0 - f64::EPSILON;
    Color::new(
        clamp(r, 0.0, n0999),
        clamp(g, 0.0, n0999),
        clamp(b, 0.0, n0999),
    )
}

fn main() {
    let camera = Camera::new(16.0 / 9.0, 2.0, 0.8, Point3::new(0, 0, 0));
    let image_width: u32 = 384;
    let image_height: u32 = (image_width as f64 / camera.aspect_ratio()) as u32;
    let mut img = image::RgbImage::new(image_width, image_height);
    let max_depth = 50;

    let mut world = HittableList::new();
    world.add(Sphere::new(
        Point3::new(0, 0, -1),
        0.5,
        Lambertian::new(Color::new(0.7, 0.3, 0.3)),
    ));
    world.add(Sphere::new(
        Point3::new(0, -100.5, -1),
        100.0,
        Lambertian::new(Color::new(0.8, 0.8, 0.0)),
    ));
    world.add(Sphere::new(
        Point3::new(1, 0, -1),
        0.5,
        Metal::new(Color::new(0.8, 0.6, 0.2), 1.0),
    ));
    world.add(Sphere::new(
        Point3::new(-1, 0, -1),
        0.5,
        Metal::new(Color::new(0.8, 0.8, 0.8), 0.3),
    ));

    let samples_per_pixel: usize = 100;
    let mut rng: Box<dyn RngCore> = Box::new(rand::thread_rng());

    let bar = ProgressBar::new((image_width * image_height).into());
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let y = image_height - y;
        let mut color = Color::new(0, 0, 0);
        for _ in 0..samples_per_pixel {
            let u = (x as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
            let v = (y as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
            let r = camera.get_ray(u, v);
            color += ray_color(r, &world, max_depth, &mut rng);
        }
        *pixel = scale_color(color, samples_per_pixel as u32).to_rgb();
        bar.inc(1)
    }
    bar.finish();
    img.save("result.png").unwrap();
}
