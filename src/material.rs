pub mod lambertian;
pub mod metal;

use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

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
