pub mod ray;
pub mod vec3;

use ray::Ray;
use vec3::Color;

fn main() {
    let mut img = image::RgbImage::new(256, 256);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color: Color = Color::new((x as f64) / 256.0, (y as f64) / 256.0, 0);
        *pixel = color.to_rgb()
    }
    img.save("result.png").unwrap();
}
