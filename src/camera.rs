use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    focal_length: f64,
    origin: Point3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64, origin: Point3) -> Self {
        Self {
            aspect_ratio,
            viewport_height,
            focal_length,
            origin,
        }
    }
    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }
    pub fn viewport_height(&self) -> f64 {
        self.viewport_height
    }
    pub fn viewport_width(&self) -> f64 {
        self.viewport_height() * self.aspect_ratio()
    }
    pub fn focal_length(&self) -> f64 {
        self.focal_length
    }
    pub fn origin(&self) -> Point3 {
        self.origin
    }
    pub fn horizontal(&self) -> Vec3 {
        Vec3::new(self.viewport_width(), 0, 0)
    }
    pub fn vertical(&self) -> Vec3 {
        Vec3::new(0, self.viewport_height(), 0)
    }
    pub fn lower_left_corner(&self) -> Point3 {
        self.origin
            - self.horizontal() / 2
            - self.vertical() / 2
            - Vec3::new(0, 0, self.focal_length)
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner() + self.horizontal() * u + self.vertical() * v - self.origin(),
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(16.0 / 9.0, 2.0, 1.0, Point3::new(0, 0, 0))
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Point3;

    use super::Camera;

    #[test]
    fn default_aspect_ratio() {
        assert_eq!(Camera::default().aspect_ratio(), 16.0 / 9.0)
    }

    #[test]
    fn default_viewport_height() {
        assert_eq!(Camera::default().viewport_height(), 2.0)
    }

    #[test]
    fn default_viewport_width() {
        assert_eq!(Camera::default().viewport_width(), 2.0 * 16.0 / 9.0)
    }

    #[test]
    fn default_focal_length() {
        assert_eq!(Camera::default().focal_length(), 1.0)
    }

    #[test]
    fn default_origin() {
        assert_eq!(Camera::default().origin(), Point3::new(0, 0, 0))
    }
}
