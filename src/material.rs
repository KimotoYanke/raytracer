use rand::thread_rng;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

pub trait Material: MaterialClone {
    fn scatter(self: &Self, r_in: &Ray, rec: &mut HitRecord) -> (Color, Ray)
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

#[derive(Default, Clone)]
pub struct Lambertian {
    albedo: Color,
}
impl<T: 'static + Material + Clone> MaterialClone for T {
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &mut HitRecord) -> (Color, Ray) {
        let scatter_direction = rec.normal + Vec3::random_unit_vector(&mut thread_rng());
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        (attenuation, scattered)
    }
}
