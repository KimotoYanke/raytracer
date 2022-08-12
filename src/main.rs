pub mod ray;
pub mod vec3;

use ray::Ray;
use vec3::Color;

use crate::vec3::{Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - (discriminant.sqrt())) / a;
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(&Point3::new(0, 0, -1), 0.5, &r);
    if t > 0.0 {
        let n: Vec3 = (r.at(t) - Vec3::new(0, 0, -1)).unit();
        return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (Color::new(1.0, 1.0, 1.0)) * (1.0 - t) + (Color::new(0.5, 0.7, 1.0) * t);
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 384;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Point3::new(0, 0, 0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0, 0);
    let vertical = Vec3::new(0, VIEWPORT_HEIGHT, 0);
    let lower_left_corner =
        origin.clone() - horizontal / 2 - vertical / 2 - Vec3::new(0, 0, FOCAL_LENGTH);

    let mut img = image::RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let y = IMAGE_HEIGHT - 1 - y;
        let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
        let v = y as f64 / (IMAGE_HEIGHT - 1) as f64;
        let r = Ray::new(
            origin,
            lower_left_corner + horizontal * u + vertical * v - origin,
        );
        let color: Color = ray_color(r);
        *pixel = color.to_rgb();
    }
    img.save("result.png").unwrap();
}
