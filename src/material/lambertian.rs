use rand::thread_rng;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::Material;

#[derive(Default, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &mut HitRecord) -> (Color, Ray, bool) {
        let scatter_direction = rec.normal + Vec3::random_unit_vector(&mut thread_rng());
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        (attenuation, scattered, true)
    }
}
