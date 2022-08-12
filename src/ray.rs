use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(point: Point3, dir: Vec3) -> Ray {
        Ray {
            orig: point,
            dir: dir,
        }
    }

    pub fn origin(&self) -> &Point3 {
        return &self.orig;
    }

    pub fn direction(&self) -> &Vec3 {
        return &self.dir;
    }

    pub fn at<T: Into<f64>>(&self, t: T) -> Point3 {
        return &self.orig + &(&self.dir * t.into());
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::vec3::{Point3, Vec3};

    #[test]
    fn origin() {
        let point = Point3::new(1, 2, 3);
        let ray = Ray::new(point.clone(), Vec3::new(3, 1, 2));

        assert_eq!(*ray.origin(), point)
    }

    #[test]
    fn dir() {
        let dir = Vec3::new(1, 2, 3);
        let ray = Ray::new(Vec3::new(3, 1, 2), dir.clone());

        assert_eq!(*ray.direction(), dir)
    }
}
