use rand::thread_rng;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::Material;

#[derive(Default, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(self: &Self, r_in: &Ray, rec: &mut HitRecord) -> (Color, Ray, bool) {
        let reflected = r_in.direction().unit().reflect(&rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_unit_vector(&mut thread_rng()) * self.fuzz,
        );
        let attenuation = self.albedo;
        let flg = scattered.direction().dot(&rec.normal) > 0.0;
        return (attenuation, scattered, flg);
    }
}
