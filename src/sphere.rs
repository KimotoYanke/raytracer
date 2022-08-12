use crate::{
    hittable::{HitRecord, Hittable},
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}
impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn center(&self) -> &Point3 {
        &self.center
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let center = self.center();
        let radius = self.radius();
        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                rec.normal = (rec.p - *center) / radius;
                return true;
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                rec.normal = (rec.p - *center) / radius;
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::Sphere;
    use crate::vec3::Point3;

    #[test]
    fn radius() {
        assert_eq!(Sphere::new(Point3::new(0, 0, 0), 10.0).radius(), 10.0)
    }

    #[test]
    fn center() {
        assert_eq!(
            *Sphere::new(Point3::new(1, 2, 3), 10.0).center(),
            Point3::new(1, 2, 3)
        )
    }
}