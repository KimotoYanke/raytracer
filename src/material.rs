use rand::thread_rng;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

pub trait Material: MaterialClone {
    fn scatter(self: &Self, r_in: &Ray, rec: &mut HitRecord) -> (Color, Ray, bool)
    where
        Color: Sized,
        Ray: Sized;
}

pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

impl<T: 'static + Material + Clone> MaterialClone for T {
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

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

#[derive(Default, Clone)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(self: &Self, r_in: &Ray, rec: &mut HitRecord) -> (Color, Ray, bool) {
        let reflected = r_in.direction().unit().reflect(&rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        let flg = scattered.direction().dot(&rec.normal) > 0.0;
        return (attenuation, scattered, flg);
    }
}
