use std::{fmt, ops};

#[derive(Debug, PartialEq, Clone, Default, Copy)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new<T1: Into<f64>, T2: Into<f64>, T3: Into<f64>>(x: T1, y: T2, z: T3) -> Self {
        Vec3(x.into(), y.into(), z.into())
    }

    pub fn x(&self) -> f64 {
        return self.0;
    }
    pub fn y(&self) -> f64 {
        return self.1;
    }
    pub fn z(&self) -> f64 {
        return self.2;
    }

    pub fn length_squared(&self) -> f64 {
        return self.dot(&self);
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        return self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z();
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        return Vec3(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        );
    }

    pub fn unit(&self) -> Self {
        return self / self.length();
    }

    pub fn to_rgb(&self) -> image::Rgb<u8> {
        let n256 = 256.0;
        return image::Rgb([
            (self.x() * n256) as u8,
            (self.y() * n256) as u8,
            (self.z() * n256) as u8,
        ]);
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        return Vec3(-self.0, -self.1, -self.2);
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        return Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        return &self + &rhs;
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        return Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        return &self - &rhs;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl<T: Into<f64>> ops::Mul<T> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        return Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}

impl<T: Into<f64>> ops::Mul<T> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: T) -> Self::Output {
        return &self * rhs;
    }
}

impl<T: Into<f64>> ops::MulAssign<T> for Vec3 {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl<T: Into<f64>> ops::Div<T> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        return Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs);
    }
}

impl<T: Into<f64>> ops::Div<T> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: T) -> Self::Output {
        return &self / rhs;
    }
}

impl<T: Into<f64>> ops::DivAssign<T> for Vec3 {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} {:.2} {:.2}", self.x(), self.y(), self.z())
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn xyz() {
        let vec = Vec3::new(0, 1, 2);
        assert_eq!(vec.x(), 0.0);
        assert_eq!(vec.y(), 1.0);
        assert_eq!(vec.z(), 2.0);
    }

    #[test]
    fn length() {
        let vec = Vec3::new(0, 3, 4);
        assert_eq!(vec.length(), 5.0);
        assert_eq!(vec.length_squared(), 25.0);
    }

    #[test]
    fn neg() {
        assert_eq!(-Vec3::new(0, 1, 2), Vec3::new(0, -1, -2));
    }

    #[test]
    fn add() {
        assert_eq!(
            &Vec3::new(0, 1.0, 2.0) + &Vec3::new(2.0, 1.0, 0.0),
            Vec3::new(2.0, 2.0, 2.0)
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            &Vec3::new(0.0, 1.0, 2.0) - &Vec3::new(2.0, 1.0, 0.0),
            Vec3::new(-2.0, 0.0, 2.0)
        );
    }

    #[test]
    fn add_assign() {
        let mut a = Vec3::new(0.0, 1.0, 2.0);
        a += Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(a, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn sub_assign() {
        let mut a = Vec3::new(0.0, 1.0, 2.0);
        a -= Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(a, Vec3::new(-2.0, 0.0, 2.0));
    }

    #[test]
    fn mul() {
        assert_eq!(&Vec3::new(0.0, 1.0, 2.0) * 2, Vec3::new(0.0, 2.0, 4.0))
    }

    #[test]
    fn mul_assign() {
        let mut a = Vec3::new(0.0, 1.0, 2.0);
        a *= 2;
        assert_eq!(a, Vec3::new(0.0, 2.0, 4.0))
    }

    #[test]
    fn div() {
        assert_eq!(&Vec3::new(0.0, 1.0, 2.0) / 2, Vec3::new(0.0, 0.5, 1.0))
    }

    #[test]
    fn div_assign() {
        let mut a = Vec3::new(0.0, 1.0, 2.0);
        a /= 2;
        assert_eq!(a, Vec3::new(0.0, 0.5, 1.0))
    }

    #[test]
    fn dot() {
        assert_eq!(Vec3::new(0.0, 1.0, 2.0).dot(&Vec3::new(1.0, 0.0, 1.0)), 2.0)
    }

    #[test]
    fn cross() {
        assert_eq!(
            Vec3::new(0.0, 1.0, 2.0).cross(&Vec3::new(1.0, 0.0, 1.0)),
            Vec3::new(1.0, 2.0, -1.0)
        )
    }

    #[test]
    fn unit_vector() {
        assert_eq!(
            Vec3::new(0.0, 3.0, 4.0).unit(),
            Vec3::new(0.0, 3.0 / 5.0, 4.0 / 5.0)
        )
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Vec3::new(0.0, 1.0, 12.0)), "0.00 1.00 12.00")
    }

    #[test]
    fn to_rgb() {
        assert_eq!(Vec3::new(0.0, 0.5, 1.0).to_rgb(), image::Rgb([0, 128, 255]))
    }
}
