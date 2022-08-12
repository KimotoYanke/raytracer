pub mod vec3;

use vec3::Vec3;

fn main() {
    let mut img = image::RgbImage::new(256, 256);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color = Vec3::new((x as f64) / 256.0, (y as f64) / 256.0, 0);
        *pixel = color.to_rgb()
    }
    img.save("result.png").unwrap();
}
