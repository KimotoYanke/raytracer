use crate::{ray::Ray, vec3::Color};

use super::Material;

#[derive(Default, Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(
        self: &Self,
        r_in: &Ray,
        rec: &mut crate::hittable::HitRecord,
        _rng: &mut Box<dyn rand::RngCore>,
    ) -> (Color, Ray, bool)
    where
        Color: Sized,
        Ray: Sized,
    {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let r_in_unit = r_in.direction().unit();
        let refracted = r_in_unit.refract(&rec.normal.unit(), etai_over_etat);

        let scattered = Ray::new(rec.p, refracted);

        (attenuation, scattered, true)
    }
}
