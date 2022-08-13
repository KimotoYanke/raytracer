use crate::{
    material::{lambertian::Lambertian, Material},
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Box<dyn Material + 'static>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    fn new<M: Material + 'static>(
        p: Point3,
        normal: Vec3,
        material: M,
        t: f64,
        front_face: bool,
    ) -> HitRecord {
        HitRecord {
            p,
            normal,
            material: Box::new(material),
            t,
            front_face,
        }
    }

    pub fn set_face_normal(self: &mut Self, r: &Ray, outward_normal: &Vec3) {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new(
            Point3::new(0, 0, 0),
            Vec3::new(0, 0, 0),
            Lambertian::default(),
            0.0,
            true,
        )
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList { objects: vec![] }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<T: Hittable + 'a>(&mut self, t: T) {
        self.objects.push(Box::<T>::new(t))
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(
            Point3::new(0, 0, 0),
            Vec3::new(0, 0, 0),
            Lambertian::default(),
            0.0,
            true,
        );
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.clone_from(&temp_rec);
            }
        }

        return hit_anything;
    }
}
