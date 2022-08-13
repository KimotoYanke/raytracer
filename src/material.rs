use rand::Rng;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

pub trait Material {
    fn scatter<T: Rng>(&self, r_in: &Ray, rec: &mut HitRecord, rng: &mut T) -> (Color, Ray);
}

pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter<T: Rng>(&self, _: &Ray, rec: &mut HitRecord, rng: &mut T) -> (Color, Ray) {
        let scatter_direction = rec.normal + Vec3::random_unit_vector(rng);
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        (attenuation, scattered)
    }
}
