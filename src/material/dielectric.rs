use rand::Rng;

use crate::{ray::Ray, vec3::Color};

use super::Material;

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0));
}

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
        rng: &mut Box<dyn rand::RngCore>,
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
        let cos_theta = -r_in_unit.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = r_in_unit.reflect(&rec.normal);
            let scattered = Ray::new(rec.p, reflected);
            return (attenuation, scattered, true);
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if rng.gen::<f64>() < reflect_prob {
            let reflected = r_in_unit.reflect(&rec.normal);
            let scattered = Ray::new(rec.p, reflected);
            return (attenuation, scattered, true);
        }

        let refracted = r_in_unit.refract(&rec.normal.unit(), etai_over_etat);

        let scattered = Ray::new(rec.p, refracted);

        (attenuation, scattered, true)
    }
}
